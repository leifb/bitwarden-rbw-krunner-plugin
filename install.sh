#!/bin/bash

set -e

# compile
cargo b -r

# Install plugin and dbus service files
mkdir -p ~/.local/share/krunner/dbusplugins/
mkdir -p ~/.local/share/dbus-1/services/
mkdir -p ~/.config/bitwarden-rbw-krunner/

cp bitwarden-rbw-krunner.desktop ~/.local/share/krunner/dbusplugins/
sed "s|%{PROJECTDIR}|${PWD}|" "de.leifb.BitwardenRbwKrunner.service" > ~/.local/share/dbus-1/services/de.leifb.BitwardenRbwKrunner.service

# link config file
if [[ ! -e ~/.config/bitwarden-rbw-krunner/config.toml ]]; then
  cp config.example.toml ~/.config/bitwarden-rbw-krunner/config.toml
fi

kquitapp6 krunner # restart krunner
pkill -f bitwarden-rbw-krunner # restart this plugin
