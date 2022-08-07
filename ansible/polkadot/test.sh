#!/bin/sh
#
# Prerequisites:
#
# - Ansible (`brew install ansible`)
# - Private key in `.ansible_key` file
# - Inventory in `.ansible_inventory` file:
#     alice-host name=alice node_key=0000000000000000000000000000000000000000000000000000000000000001
#     bob-host name=bob bootnode=/dns4/alice-host/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
#

set -e
cd "$(dirname "$0")"

ansible-playbook install.yml --inventory-file .ansible_inventory -u ubuntu --key-file .ansible_key
