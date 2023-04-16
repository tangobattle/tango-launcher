use futures_util::SinkExt;
use futures_util::TryStreamExt;
use prost::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::version;

async fn create_data_channel(
    rtc_config: datachannel_wrapper::RtcConfig,
) -> Result<
    (
        datachannel_wrapper::DataChannel,
        tokio::sync::mpsc::Receiver<datachannel_wrapper::PeerConnectionEvent>,
        datachannel_wrapper::PeerConnection,
    ),
    anyhow::Error,
> {
    let (mut peer_conn, mut event_rx) = datachannel_wrapper::PeerConnection::new(rtc_config)?;

    let dc = peer_conn.create_data_channel(
        "tango",
        datachannel_wrapper::DataChannelInit::default()
            .reliability(datachannel_wrapper::Reliability {
                unordered: false,
                unreliable: false,
                max_packet_life_time: 0,
                max_retransmits: 0,
            })
            .negotiated()
            .manual_stream()
            .stream(0),
    )?;

    loop {
        if let Some(datachannel_wrapper::PeerConnectionEvent::GatheringStateChange(
            datachannel_wrapper::GatheringState::Complete,
        )) = event_rx.recv().await
        {
            break;
        }
    }

    Ok((dc, event_rx, peer_conn))
}

pub struct PendingConnection {
    signaling_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    dc: datachannel_wrapper::DataChannel,
    event_rx: tokio::sync::mpsc::Receiver<datachannel_wrapper::PeerConnectionEvent>,
    peer_conn: datachannel_wrapper::PeerConnection,
}

pub async fn open(addr: &str, session_id: &str, use_relay: Option<bool>) -> Result<PendingConnection, anyhow::Error> {
    let mut url = url::Url::parse(addr)?;
    url.set_query(Some(
        &url::form_urlencoded::Serializer::new(String::new())
            .append_pair("session_id", session_id)
            .finish(),
    ));

    let mut req = url.to_string().into_client_request()?;
    req.headers_mut().append(
        "User-Agent",
        tokio_tungstenite::tungstenite::http::HeaderValue::from_str(&format!("tango/{}", version::current()))?,
    );
    let (mut signaling_stream, _) = tokio_tungstenite::connect_async(req).await?;

    let raw = if let Some(raw) = signaling_stream.try_next().await? {
        raw
    } else {
        anyhow::bail!("stream ended early");
    };

    let packet = if let tokio_tungstenite::tungstenite::Message::Binary(d) = raw {
        tango_protos::matchmaking::Packet::decode(bytes::Bytes::from(d))?
    } else {
        anyhow::bail!("invalid packet");
    };

    let hello = if let Some(tango_protos::matchmaking::packet::Which::Hello(hello)) = packet.which {
        hello
    } else {
        anyhow::bail!("invalid packet");
    };

    log::info!("hello received from signaling stream: {:?}", hello);

    let mut rtc_config = datachannel_wrapper::RtcConfig::new(
        &hello
            .ice_servers
            .into_iter()
            .flat_map(|ice_server| {
                ice_server
                    .urls
                    .into_iter()
                    .flat_map(|url| {
                        let colon_idx = if let Some(colon_idx) = url.chars().position(|c| c == ':') {
                            colon_idx
                        } else {
                            return vec![];
                        };

                        let proto = &url[..colon_idx];
                        let rest = &url[colon_idx + 1..];

                        if (proto == "turn" || proto == "turns") && use_relay == Some(false) {
                            return vec![];
                        }

                        // libdatachannel doesn't support TURN over TCP: in fact, it explodes!
                        if url.chars().skip_while(|c| *c != '?').collect::<String>() == "?transport=tcp" {
                            return vec![];
                        }

                        if let (Some(username), Some(credential)) = (&ice_server.username, &ice_server.credential) {
                            vec![format!(
                                "{}:{}:{}@{}",
                                proto,
                                urlencoding::encode(username),
                                urlencoding::encode(credential),
                                rest
                            )]
                        } else {
                            vec![format!("{}:{}", proto, rest)]
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    if use_relay == Some(true) {
        rtc_config.ice_transport_policy = datachannel_wrapper::TransportPolicy::Relay;
    }
    let (dc, event_rx, peer_conn) = create_data_channel(rtc_config).await?;

    signaling_stream
        .send(tokio_tungstenite::tungstenite::Message::Binary(
            tango_protos::matchmaking::Packet {
                which: Some(tango_protos::matchmaking::packet::Which::Start(
                    tango_protos::matchmaking::packet::Start {
                        protocol_version: crate::net::protocol::VERSION as u32,
                        offer_sdp: peer_conn.local_description().unwrap().sdp.to_string(),
                    },
                )),
            }
            .encode_to_vec(),
        ))
        .await?;

    Ok(PendingConnection {
        signaling_stream,
        dc,
        event_rx,
        peer_conn,
    })
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("signaling abort: {0:?}")]
    ServerAbort(tango_protos::matchmaking::packet::Abort),

    #[error("tungstenite: {0:?}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("prost decode error: {0:?}")]
    ProstDecode(#[from] prost::DecodeError),

    #[error("sdp parse error: {0:?}")]
    SdpParse(#[from] datachannel_wrapper::sdp::error::SdpParserError),

    #[error("{0:?}")]
    Other(#[from] anyhow::Error),

    #[error("stream ended early")]
    StreamEndedEarly,

    #[error("invalid packet")]
    InvalidPacket(tokio_tungstenite::tungstenite::Message),

    #[error("unexpected packet: {0:?}")]
    UnexpectedPacket(tango_protos::matchmaking::Packet),

    #[error("peer connection unexpectedly disconnected")]
    PeerConnectionDisconnected,

    #[error("peer connection failed")]
    PeerConnectionFailed,

    #[error("peer connection unexpectedly closed")]
    PeerConnectionClosed,
}

impl PendingConnection {
    pub async fn connect(
        mut self,
    ) -> Result<(datachannel_wrapper::DataChannel, datachannel_wrapper::PeerConnection), Error> {
        loop {
            let raw = if let Some(raw) = self.signaling_stream.try_next().await? {
                raw
            } else {
                return Err(Error::StreamEndedEarly);
            };

            let packet = if let tokio_tungstenite::tungstenite::Message::Binary(d) = raw {
                tango_protos::matchmaking::Packet::decode(bytes::Bytes::from(d))?
            } else {
                return Err(Error::InvalidPacket(raw));
            };

            match &packet.which {
                Some(tango_protos::matchmaking::packet::Which::Abort(abort)) => {
                    return Err(Error::ServerAbort(abort.clone()))
                }
                Some(tango_protos::matchmaking::packet::Which::Offer(offer)) => {
                    log::info!("received an offer, this is the polite side. rolling back our local description and switching to answer");

                    self.peer_conn
                        .set_local_description(datachannel_wrapper::SdpType::Rollback)?;
                    self.peer_conn
                        .set_remote_description(datachannel_wrapper::SessionDescription {
                            sdp_type: datachannel_wrapper::SdpType::Offer,
                            sdp: datachannel_wrapper::sdp::parse_sdp(&offer.sdp.to_string(), false)?,
                        })?;

                    let local_description = self.peer_conn.local_description().unwrap();
                    self.signaling_stream
                        .send(tokio_tungstenite::tungstenite::Message::Binary(
                            tango_protos::matchmaking::Packet {
                                which: Some(tango_protos::matchmaking::packet::Which::Answer(
                                    tango_protos::matchmaking::packet::Answer {
                                        sdp: local_description.sdp.to_string(),
                                    },
                                )),
                            }
                            .encode_to_vec(),
                        ))
                        .await?;
                    log::info!("sent answer to impolite side");
                    break;
                }
                Some(tango_protos::matchmaking::packet::Which::Answer(answer)) => {
                    log::info!("received an answer, this is the impolite side");

                    self.peer_conn
                        .set_remote_description(datachannel_wrapper::SessionDescription {
                            sdp_type: datachannel_wrapper::SdpType::Answer,
                            sdp: datachannel_wrapper::sdp::parse_sdp(&answer.sdp, false)?,
                        })?;
                    break;
                }
                _ => {
                    return Err(Error::UnexpectedPacket(packet));
                }
            }
        }

        self.signaling_stream.close(None).await?;

        log::debug!(
            "local sdp (type = {:?}): {}",
            self.peer_conn.local_description().expect("local sdp").sdp_type,
            self.peer_conn.local_description().expect("local sdp").sdp
        );
        log::debug!(
            "remote sdp (type = {:?}): {}",
            self.peer_conn.remote_description().expect("remote sdp").sdp_type,
            self.peer_conn.remote_description().expect("remote sdp").sdp
        );

        loop {
            match self.event_rx.recv().await {
                Some(signal) => match signal {
                    datachannel_wrapper::PeerConnectionEvent::ConnectionStateChange(c) => match c {
                        datachannel_wrapper::ConnectionState::Connected => {
                            break;
                        }
                        datachannel_wrapper::ConnectionState::Disconnected => {
                            return Err(Error::PeerConnectionDisconnected);
                        }
                        datachannel_wrapper::ConnectionState::Failed => {
                            return Err(Error::PeerConnectionFailed);
                        }
                        datachannel_wrapper::ConnectionState::Closed => {
                            return Err(Error::PeerConnectionClosed);
                        }
                        _ => {}
                    },
                    _ => {}
                },
                None => unreachable!(),
            }
        }

        Ok((self.dc, self.peer_conn))
    }
}
