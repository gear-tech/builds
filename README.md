# Builds

[![Nightly Status](https://github.com/gear-tech/builds/workflows/Nightly/badge.svg)](https://github.com/gear-tech/builds/actions/workflows/nightly.yml?query=branch%3Amaster)

Prebuilt Gear binaries.

⚓ <https://get.gear.rs>

Packages include:

- `gear`[`.exe`]

  Source code: <https://github.com/gear-tech/gear/tree/master/node/cli>

## Gear node nightly builds

- Linux x64: <https://get.gear.rs/gear-nightly-x86_64-unknown-linux-gnu.tar.xz>
- macOS M-series (ARM): <https://get.gear.rs/gear-nightly-aarch64-apple-darwin.tar.xz>
- macOS Intel x64: <https://get.gear.rs/gear-nightly-x86_64-apple-darwin.tar.xz>
- Windows x64: <https://get.gear.rs/gear-nightly-x86_64-pc-windows-msvc.zip>

## Gear node release builds

Find them at <https://get.gear.rs>.

# Installation using Homebrew (macOS)

```bash
brew tap gear-tech/gear
brew install gear         # Latest release
brew install gear@nightly # Latest nightly
brew install gear@1.0.0   # Specific version
```

# Gear node install script (Linux, macOS)

Install the latest release:

```bash
curl -sSf https://get.gear.rs/install.sh | sh
```

Install a specific version (e.g. v1.0.0):

```bash
curl -sSf https://get.gear.rs/install.sh | sh -s -- --tag v1.0.0
```
