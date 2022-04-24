import React from "react";
import { Trans } from "react-i18next";
import tmp from "tmp-promise";

import Box from "@mui/material/Box";
import CircularProgress from "@mui/material/CircularProgress";
import Modal from "@mui/material/Modal";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";

import { makeROM } from "../../game";
import * as ipc from "../../ipc";
import { ReplayInfo } from "../../replay";
import { useConfig } from "./ConfigContext";

export function CoreSupervisor({
  romPath,
  savePath,
  patchPath,
  matchSettings,
  windowTitle,
  incarnation,
  onExit,
}: {
  romPath: string;
  savePath: string;
  patchPath?: string;
  matchSettings?: {
    sessionID: string;
    replaysPath: string;
    replayInfo: ReplayInfo;
  };
  incarnation: number;
  windowTitle: string;
  onExit: (exitStatus: ipc.ExitStatus) => void;
}) {
  const { config } = useConfig();

  const configRef = React.useRef(config);
  const romTmpFileRef = React.useRef<tmp.FileResult | null>(null);

  const onExitRef = React.useRef(onExit);
  React.useEffect(() => {
    onExitRef.current = onExit;
  }, [onExit]);

  const [state, setState] = React.useState<ipc.State | null>(null);
  const [stderr, setStderr] = React.useState<string[]>([]);
  const [exitStatus, setExitStatus] = React.useState<ipc.ExitStatus | null>(
    null
  );

  const abortControllerRef = React.useRef<AbortController>(null!);
  if (abortControllerRef.current == null) {
    abortControllerRef.current = new AbortController();
  }

  React.useEffect(() => {
    (async () => {
      romTmpFileRef.current = await makeROM(romPath, patchPath || null);

      const core = new ipc.Core(
        {
          window_title: windowTitle,
          rom_path: romTmpFileRef.current!.path,
          save_path: savePath,
          keymapping: configRef.current.keymapping,
          match_settings:
            matchSettings == null
              ? null
              : {
                  session_id: matchSettings.sessionID,
                  input_delay: 0,
                  match_type: 0,
                  matchmaking_connect_addr:
                    configRef.current.matchmakingConnectAddr,
                  ice_servers: configRef.current.iceServers,
                  replays_path: matchSettings.replaysPath,
                  replay_metadata: JSON.stringify(matchSettings.replayInfo),
                },
        },
        {
          signal: abortControllerRef.current.signal,
        }
      );
      core.on("exit", (exitStatus) => {
        setExitStatus(exitStatus);
        onExitRef.current(exitStatus);
      });
      core.on("state", (state) => {
        setState(state);
      });
      core.on("stderr", (stderr) => {
        setStderr((lines) => {
          lines.push(stderr);
          return lines;
        });
      });
    })();

    return () => {
      if (romTmpFileRef.current != null) {
        romTmpFileRef.current.cleanup();
      }
      abortControllerRef.current.abort();
    };
  }, [romPath, savePath, patchPath, windowTitle, matchSettings, incarnation]);

  return (
    <Modal
      open={true}
      onClose={(e, reason) => {
        if (reason == "backdropClick" || reason == "escapeKeyDown") {
          return;
        }
      }}
    >
      <Box
        sx={{
          position: "absolute",
          top: "50%",
          left: "50%",
          transform: "translate(-50%, -50%)",
          width: 300,
          bgcolor: "background.paper",
          boxShadow: 24,
          px: 3,
          py: 2,
        }}
      >
        <Stack
          direction="row"
          justifyContent="flex-start"
          alignItems="center"
          spacing={2}
        >
          <CircularProgress
            sx={{ flexGrow: 0, flexShrink: 0 }}
            disableShrink
            size="2rem"
          />
          <Typography>
            {state == null ? (
              <Trans i18nKey="supervisor:status.starting" />
            ) : state == "Running" ? (
              <Trans i18nKey="supervisor:status.running" />
            ) : state == "Waiting" ? (
              <Trans i18nKey="supervisor:status.waiting" />
            ) : state == "Connecting" ? (
              <Trans i18nKey="supervisor:status.connecting" />
            ) : null}
          </Typography>
        </Stack>
      </Box>
    </Modal>
  );
}
