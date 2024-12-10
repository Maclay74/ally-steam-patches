#!/bin/sh

echo "Uninstalling Steam Patch..."

WORKING_FOLDER="${HOME}/steam-patch"

# Disable and remove services
systemctl --user stop steam-patch 2> /dev/null
systemctl --user disable --now steam-patch 2> /dev/null
rm -f "${HOME}/.config/systemd/user/steam-patch.service"

# Cleanup services folder
rm -rf "${WORKING_FOLDER}"