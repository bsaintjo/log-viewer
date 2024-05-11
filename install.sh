#!/bin/bash

mkdir -p /home/.local/bin
cp log-viewer /home/.local/bin
chmod 755 /home/.local/bin/log-viewer
desktop-file-install --dir=/usr/share/applications log-viewer.desktop
cp log-128x128.png /data/pattern/images
update-desktop-database