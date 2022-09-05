# Tango

Tango is rollback netplay for Mega Man Battle Network.

## Supported games

| ID                 | Name                                                  | Gameplay support            | Save viewer support                         |
| ------------------ | ----------------------------------------------------- | --------------------------- | ------------------------------------------- |
| `MEGAMAN6_FXXBR6E` | Mega Man Battle Network 6: Cybeast Falzar (US)        | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `MEGAMAN6_GXXBR5E` | Mega Man Battle Network 6: Cybeast Gregar (US)        | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `ROCKEXE6_RXXBR6J` | Rockman EXE 6: Dennoujuu Falzer (JP)                  | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards               |
| `ROCKEXE6_GXXBR5J` | Rockman EXE 6: Dennoujuu Glaga (JP)                   | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards               |
| `MEGAMAN5_TP_BRBE` | Mega Man Battle Network 5: Team Protoman (US)         | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `MEGAMAN5_TC_BRKE` | Mega Man Battle Network 5: Team Colonel (US)          | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `ROCKEXE5_TOBBRBJ` | Rockman EXE 5: Team of Blues (JP)                     | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `ROCKEXE5_TOCBRKJ` | Rockman EXE 5: Team of Colonel (JP)                   | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `ROCKEXE4.5ROBR4J` | Rockman EXE 4.5: Real Operation (JP)                  | ✅ Works great!             | ✅ Navi, Folder                             |
| `MEGAMANBN4BMB4BE` | Mega Man Battle Network 4: Blue Moon (US)             | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `MEGAMANBN4RSB4WE` | Mega Man Battle Network 4: Red Sun (US)               | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `ROCK_EXE4_BMB4BJ` | Rockman EXE 4: Tournament Blue Moon (Rev 0 only) (JP) | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `ROCK_EXE4_RSB4WJ` | Rockman EXE 4: Tournament Red Sun (Rev 1 only) (JP)   | ✅ Works great!             | 🤷 Folder, NaviCust, Modcards, Dark Soul AI |
| `MEGA_EXE3_BLA3XE` | Megaman Battle Network 3: Blue (US)                   | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `MEGA_EXE3_WHA6BE` | Megaman Battle Network 3: White (US)                  | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `ROCK_EXE3_BKA3XJ` | Battle Network Rockman EXE 3: Black (Rev 1 only) (JP) | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `ROCKMAN_EXE3A6BJ` | Battle Network Rockman EXE 3 (Rev 1 only) (JP)        | ✅ Works great!             | 🤷 Folder, NaviCust                         |
| `MEGAMAN_EXE2AE2E` | Megaman Battle Network 2 (US)                         | 🤷 Works, with minor issues | 🤷 Folder                                   |
| `ROCKMAN_EXE2AE2J` | Battle Network Rockman EXE 2 (Rev 1 only) (JP)        | 🤷 Works, with minor issues | 🤷 Folder                                   |
| `MEGAMAN_BN@@AREE` | Megaman Battle Network (US)                           | 🤷 Works, with minor issues | 🤷 Folder                                   |
| `ROCKMAN_EXE@AREJ` | Battle Network Rockman EXE (JP)                       | 🤷 Works, with minor issues | 🤷 Folder                                   |

## Building

1.  Install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1.  Install the Rust target and toolchain for `x86_64-pc-windows-gnu`.

    ```sh
    rustup target add x86_64-pc-windows-gnu
    rustup toolchain install stable-x86_64-pc-windows-gnu
    ```

1.  Install mingw-w64.

    ```sh
    sudo apt-get install -y mingw-w64
    ```

1.  Ensure mingw-w64 is using the POSIX threading model.

    ```sh
    sudo update-alternatives --install /usr/bin/x86_64-w64-mingw32-gcc x86_64-w64-mingw32-gcc /usr/bin/x86_64-w64-mingw32-gcc-win32 60 &&
    sudo update-alternatives --install /usr/bin/x86_64-w64-mingw32-gcc x86_64-w64-mingw32-gcc /usr/bin/x86_64-w64-mingw32-gcc-posix 90 &&
    sudo update-alternatives --config x86_64-w64-mingw32-gcc &&
    sudo update-alternatives --install /usr/bin/x86_64-w64-mingw32-g++ x86_64-w64-mingw32-g++ /usr/bin/x86_64-w64-mingw32-g++-win32 60 &&
    sudo update-alternatives --install /usr/bin/x86_64-w64-mingw32-g++ x86_64-w64-mingw32-g++ /usr/bin/x86_64-w64-mingw32-g++-posix 90 &&
    sudo update-alternatives --config x86_64-w64-mingw32-g++
    ```

1.  Build it.

    ```sh
    cargo build --target x86_64-pc-windows-gnu --release --bin tango
    ```

### Server

The server is the remote HTTP server-based component that Tango connects to. It doesn't actually do very much, so you can run it on absolutely piddly hardware. All it does is provide signaling by sending WebRTC SDPs around.

If you already have Rust installed, you can build it like so:

```sh
cargo build --release --bin tango-server
```

## Language support

Tango is fully internationalized and supports language switching based on your computer's language settings.

The order of language support is as follows:

-   **English (en):** This is Tango's primary and fallback language. All Tango development is done in English.

-   **Japanese (ja):** This is Tango's secondary but fully supported language. All text in the UI, barring some extremely supplementary text (e.g. the About screen) is expected to be available in Japanese. If new UI text is added, a Japanese translation SHOULD also be provided. Tango releases MUST NOT contain missing Japanese text.

-   **All other languages:** These are Tango's tertiary languages. Support is provided on a best effort basis and translations are provided as available.
