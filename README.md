# Builds

[![Nightly Status](https://github.com/gear-tech/builds/workflows/Nightly/badge.svg)](https://github.com/gear-tech/builds/actions/workflows/nightly.yml?query=branch%3Amaster)

Prebuilt Gear binaries.

Packages include:

- `gear-node`[`.exe`]

  Source code: https://github.com/gear-tech/gear/tree/master/node

## Nightly builds

- Windows x64: https://builds.gear.rs/gear-nightly-windows-x86_64.zip
- macOS M1: https://builds.gear.rs/gear-nightly-macos-m1.tar.gz
- macOS Intel x64: https://builds.gear.rs/gear-nightly-macos-x86_64.tar.gz
- Linux x64: https://builds.gear.rs/gear-nightly-linux-x86_64.tar.xz

# Ansible scripts

You can deploy Gear node using Ansible scripts.

## Install prerequisites

### Ansible

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

### (Optional) SSHPass

If you access your server using SSH login and password instead of SSH key, you are to install `sshpass` too.

- macOS:

    ```
    brew install esolitos/ipa/sshpass
    ```

- Ubuntu Linux:

    ```
    sudo apt update
    sudo apt install sshpass
    ```

## Modify variables

Refer the [ansible/gear-node/install.yml](ansible/gear-node/install.yml) config and modify `vars` according to your setup:

- `name` (default: `MY_SUPER_NODE`) is the node name that will be visible on https://telemetry.gear-tech.io/
- `port_http` (default: `9933`) is the HTTP RPC server TCP port
- `port_ws` (default: `9944`) is the WebSockets RPC server TCP port
- `port_p2p` (default: `30333`) is the P2P protocol TCP port

## Deploy Gear node

- **Preferred way:** Using a private key file for SSH access:

    ```
    ansible-playbook ansible/gear-node/install.yml -i <my-host>, -u <user> --key-file <path-to-key-file>
    ```

    Example:

    ```
    ansible-playbook ansible/gear-node/install.yml -i node.gear.rs, -u root --key-file ~/key.pem
    ```

- Using a root user login/password for SSH access:

    ```
    ansible-playbook ansible/gear-node/install.yml -i <my-host>, -u <user> -k
    ```

    Example:

    ```
    ansible-playbook ansible/gear-node/install.yml -i node.gear.rs, -u root -k
    ```
