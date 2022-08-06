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

# Ansible Scripts

You can deploy Gear node using Ansible scripts.

## Install Ansible

- macOS:

    ```
    brew install ansible
    ```

- Ubuntu Linux:

    ```
    sudo apt update
    sudo apt install software-properties-common
    sudo add-apt-repository --yes --update ppa:ansible/ansible
    sudo apt install ansible
    ```
## Deploy Gear Node

Using a private key file for SSH access:

```
ansible-playbook ansible/gear-node/install.yml -i <my-host>, -u <user> --key-file <path-to-key-file>
```

Using a root user login/password for SSH access:

```
ansible-playbook ansible/gear-node/install.yml -i <my-host>, -u <user> -k
```
