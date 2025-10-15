#!/bin/bash

set -e

rm ~/.local/share/krunner/dbusplugins/bitwarden-rbw-krunner.desktop
rm ~/.local/share/dbus-1/services/de.leifb.BitwardenRbwKrunner.service

kquitapp6 krunner
pkill -f bitwarden-rbw-krunner
