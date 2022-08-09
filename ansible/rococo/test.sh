#!/bin/sh
#
# Prerequisites:
#
# - Ansible (`brew install ansible`)
# - Private key in `.ansible_key` file
# - Inventory in `.ansible_inventory` file:
#     relay-host
#

set -e
cd "$(dirname "$0")"

ansible-playbook install.yml --inventory-file .ansible_inventory -u ubuntu --key-file .ansible_key
