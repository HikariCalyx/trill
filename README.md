![Trill_Logo](https://github.com/HikariCalyx/trill/blob/main/tango/src/emblem.png?raw=true)

# Trill

Trill is rollback netplay for Mega Man Battle Network, served as continued development from Tango.

## Why this name?

Trill is named after the NetNavi of same name in Rockman.EXE BEAST anime.

## Supported games

| Name                                                  | Gameplay support            | Save viewer support                                |
| ----------------------------------------------------- | --------------------------- | -------------------------------------------------- |
| Mega Man Battle Network 6: Cybeast Falzar (US)        | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Mega Man Battle Network 6: Cybeast Gregar (US)        | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Rockman EXE 6: Dennoujuu Falzer (JP)                  | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards                   |
| Rockman EXE 6: Dennoujuu Glaga (JP)                   | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards                   |
| Mega Man Battle Network 5: Team Protoman (US)         | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Mega Man Battle Network 5: Team Colonel (US)          | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Rockman EXE 5: Team of Blues (JP)                     | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Rockman EXE 5: Team of Colonel (JP)                   | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Rockman EXE 4.5: Real Operation (JP)                  | ✅ Works great!             | ✅ Navi, Folder                                    |
| Mega Man Battle Network 4: Blue Moon (US)             | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Mega Man Battle Network 4: Red Sun (US)               | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Rockman EXE 4: Tournament Blue Moon (Rev 1 only) (JP) | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Rockman EXE 4: Tournament Red Sun (Rev 1 only) (JP)   | ✅ Works great!             | 🤷 Folder, NaviCust, Patch Cards, Auto Battle Data |
| Megaman Battle Network 3: Blue (US)                   | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Megaman Battle Network 3: White (US)                  | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Battle Network Rockman EXE 3: Black (Rev 1 only) (JP) | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Battle Network Rockman EXE 3 (Rev 1 only) (JP)        | ✅ Works great!             | 🤷 Folder, NaviCust                                |
| Megaman Battle Network 2 (US)                         | 🤷 Works, with minor issues | 🤷 Folder                                          |
| Battle Network Rockman EXE 2 (AdColle only) (JP)      | 🤷 Works, with minor issues | 🤷 Folder                                          |
| Megaman Battle Network (US)                           | 🤷 Works, with minor issues | 🤷 Folder                                          |
| Battle Network Rockman EXE (JP)                       | 🤷 Works, with minor issues | 🤷 Folder                                          |

## Building (Linux Binary)

We assume you're using Ubuntu or Debian.

1.  Install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1.  Install required dependencies.

    ```sh
    sudo apt-get install -y libssl-dev libglib2.0-dev pkg-config cmake build-essential libclang-dev libgtk-3-dev librust-alsa-sys-dev libasound2-dev curl wget git libfuse2
    ```

1.  Clone this repository.

    ```sh
    git clone --recurse-submodules https://github.com/HikariCalyx/trill
    cd trill
    ```

1.  Run the build script. It will create an AppImage in the dist directory.

    ```sh
    bash ./linux/build.sh
    # For ARM32 Build: bash ./linux/build_armhf.sh
    # For ARM64 Build: bash ./linux/build_arm64.sh
    ```

## Building (Windows Binary)

1.  Install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1.  Install the Rust target and toolchain for `x86_64-pc-windows-gnu`.

    ```sh
    rustup target add x86_64-pc-windows-gnu
    rustup toolchain install stable-x86_64-pc-windows-gnu
    ```

1.  Install mingw-w64 and all other required dependencies.

    ```sh
    sudo apt-get install -y libssl-dev libglib2.0-dev pkg-config cmake build-essential libclang-dev libgtk-3-dev librust-alsa-sys-dev libasound2-dev curl wget git mingw-w64 clang nsis python3-pip python3-dev p7zip-full imagemagick libarchive-tools
    pip install mako semver toml
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

1.  Clone this repository.

    ```sh
    git clone --recurse-submodules https://github.com/HikariCalyx/trill
    cd trill
    ```

1.  Build it.

    ```sh
    bash ./win/build.sh
    ```

## Building (32Bit Windows Binary)

The result made by Debian 11 amd64 is guaranteed to work.

1.  Install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1.  Install the Rust target and toolchain for `i686-pc-windows-gnu`.

    ```sh
    rustup target add i686-pc-windows-gnu
    rustup toolchain install stable-i686-pc-windows-gnu
    ```

1.  Install mingw-w64 and all other required dependencies.

    ```sh
    sudo apt-get install -y libssl-dev libglib2.0-dev pkg-config cmake build-essential libclang-dev libgtk-3-dev librust-alsa-sys-dev libasound2-dev curl wget git mingw-w64 clang nsis python3-pip python3-dev p7zip-full imagemagick libarchive-tools
    pip install mako semver toml
    ```

1.  Ensure mingw-w64 is using the POSIX threading model.

    ```sh
    sudo update-alternatives --install /usr/bin/i686-w64-mingw32-gcc i686-w64-mingw32-gcc /usr/bin/i686-w64-mingw32-gcc-win32 60 &&
    sudo update-alternatives --install /usr/bin/i686-w64-mingw32-gcc i686-w64-mingw32-gcc /usr/bin/i686-w64-mingw32-gcc-posix 90 &&
    sudo update-alternatives --config i686-w64-mingw32-gcc &&
    sudo update-alternatives --install /usr/bin/i686-w64-mingw32-g++ i686-w64-mingw32-g++ /usr/bin/i686-w64-mingw32-g++-win32 60 &&
    sudo update-alternatives --install /usr/bin/i686-w64-mingw32-g++ i686-w64-mingw32-g++ /usr/bin/i686-w64-mingw32-g++-posix 90 &&
    sudo update-alternatives --config i686-w64-mingw32-g++
    ```

1.  Clone this repository.

    ```sh
    git clone --recurse-submodules https://github.com/HikariCalyx/trill
    cd trill
    ```

1.  Build the installer.

    ```sh
    bash ./win/build_i686.sh
    ```


## Building (macOS)

You're required to use macOS 11 or newer, due to Universal 2 binary support is implemented since this version.

1. [Install Homebrew](https://brew.sh/).

1. Install Rust.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. Install the Rust target for the opposite architecture from your current Mac.

    ```sh
    # For Apple Silicon Mac, install the x86_64 target
    rustup target add x86_64-apple-darwin
    # For Intel Mac, install the aarch64 target
    rustup target add aarch64-apple-darwin
    ```

1.  Install required dependencies.

    ```sh
    brew install wget
    sudo pip3 install semver==3.0.0-dev3 toml dmgbuild pyobjc-framework-Quartz==10.3.2 mako pyobjc-core==10.3.2
    # Since homebrew has removed legacy version of cmake (3.18.4), we need to install manually.
    curl -LO https://github.com/Kitware/CMake/releases/download/v3.18.4/cmake-3.18.4.tar.gz
    tar -xzvf cmake-3.18.4.tar.gz
    cd cmake-3.18.4
    ./bootstrap
    make -j$(sysctl -n hw.logicalcpu)
    sudo make install
    ```

1.  Clone this repository.

    ```sh
    git clone --recurse-submodules https://github.com/HikariCalyx/trill
    cd trill
    ```

1.  Build the DMG.

    ```sh
    bash ./macos/build.sh
    ```

### Server

The server is the remote HTTP server-based component that Tango connects to. It doesn't actually do very much, so you can run it on absolutely piddly hardware. All it does is provide signaling by sending WebRTC SDPs around.

If you already have Rust and Perl installed ([on Windows, try Strawberry Perl](https://strawberryperl.com/)), you can build it like so:

```sh
cargo build --release --bin tango-signaling-server
```

Or, you can download prebuilt server binary from here: https://github.com/HikariCalyx/tango-matchmaking-server/releases/latest

## Language support

Trill is fully internationalized and supports language switching based on your computer's language settings.

The order of language support is as follows:

- **English (en):** This is Trill's primary and fallback language. All Trill development is done in English.

- **Japanese (ja):** This is Trill's secondary but fully supported language. All text in the UI, barring some extremely supplementary text (e.g. the About screen) is expected to be available in Japanese. If new UI text is added, a Japanese translation SHOULD also be provided. Trill releases MUST NOT contain missing Japanese text.

- **All other languages:** These are Trill's tertiary languages. Support is provided on a best effort basis and translations are provided as available.
