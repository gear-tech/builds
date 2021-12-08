# Builds

[![Nightly Status](https://github.com/gear-tech/builds/workflows/Nightly/badge.svg)](https://github.com/gear-tech/builds/actions/workflows/nightly.yml?query=branch%3Amaster)

Prebuilt Gear binaries.

Packages include:

- `gear-node`[`.exe`]

  Source code: https://github.com/gear-tech/gear/tree/master/node

## Nightly Builds

- Windows x64: https://builds.gear.rs/gear-nightly-windows-x86_64.zip
- macOS M1: https://builds.gear.rs/gear-nightly-macos-m1.tar.gz
- macOS Intel x64: https://builds.gear.rs/gear-nightly-macos-x86_64.tar.gz
- Linux x64: https://builds.gear.rs/gear-nightly-linux-x86_64.tar.xz
- Gear Runtime Wasm: https://builds.gear.rs/gear-runtime-nightly-wasm.tar.xz

## Docker Images

- Gear node (running in dev mode): https://github.com/orgs/gear-tech/packages/container/package/builds%2Fnode
- Website backend: https://github.com/orgs/gear-tech/packages/container/package/builds%2Fbackend
- Website frontend: https://github.com/orgs/gear-tech/packages/container/package/builds%2Ffrontend

## Running GUI Locally

```bash
git clone https://github.com/gear-tech/builds.git
docker-compose -f builds/docker/gui-dev.yml up
```

Then open http://localhost:3000/ in the browser.
