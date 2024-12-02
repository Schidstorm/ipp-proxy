#!/bin/bash

set -euo pipefail

echo "deb https://www.bchemnet.com/suldr/ debian extra" | sudo tee -a /etc/apt/sources.list
sudo apt-get update
wget https://www.bchemnet.com/suldr/pool/debian/extra/su/suldr-keyring_2_all.deb

if ! echo "2d996f611648a1a0a2926ceea1493ce1f29f5c1ee9ed604c61f60b87856339ae  suldr-keyring_2_all.deb" | sha256sum --check --status; then
    echo "Checksum failed for suldr-keyring_2_all.deb"
    exit 1
fi

sudo dpkg -i suldr-keyring_2_all.deb
rm suldr-keyring_2_all.deb
sudo apt-get update
sudo apt-get install -y suld-driver2-1.00.39hp

#remove from sources.list
sudo sed -i '/bchemnet.com/d' /etc/apt/sources.list
sudo apt-get update

# install ipp-proxy dependencies
sudo apt-get install -y \
    libcups2-dev 

# configure samsung printer
# install vnc and do over ui. hassle free
# https://blog.berrybase.de/raspberry-pi-fernsteuern-so-profitierst-du-von-vnc/