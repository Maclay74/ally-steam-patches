#!/bin/sh

WORKING_FOLDER="${HOME}/steam-patch"

if [ ! -d "${WORKING_FOLDER}" ]; then
  mkdir -p "${WORKING_FOLDER}"
  echo "Created ${WORKING_FOLDER}..."
fi

echo "Downloading Steam Patch..."
# Download latest release and install it
RELEASE=$(curl -s 'https://api.github.com/repos/Maclay74/steam-patch/releases' | jq -r "first(.[] | select(.prerelease == "false"))")
VERSION=$(jq -r '.tag_name' <<< ${RELEASE} )
DOWNLOAD_URL=$(jq -r '.assets[].browser_download_url | select(endswith("steam-patch"))' <<< ${RELEASE})

printf "Installing version %s...\n" "${VERSION}"
curl -L $DOWNLOAD_URL --output ${WORKING_FOLDER}/steam-patch
chmod +x ${WORKING_FOLDER}/steam-patch

echo "Enabling Steam Debugging..."
touch "${HOME}/.steam/steam/.cef-enable-remote-debugging"

echo "Creating systemd service..."
systemctl --user stop steam-patch 2> /dev/null
systemctl --user disable steam-patch 2> /dev/null

# Add new service file
cat > "${WORKING_FOLDER}/steam-patch.service" <<- EOM
[Unit]
Description=Steam Patches Loader
Wants=network.target
After=network.target

[Service]
Type=simple
ExecStart=${WORKING_FOLDER}/steam-patch
WorkingDirectory=${WORKING_FOLDER}
Restart=always

[Install]
WantedBy=default.target
EOM

rm -f "${HOME}/.config/systemd/user/steam-patch.service"
cp "${WORKING_FOLDER}/steam-patch.service" "${HOME}/.config/systemd/user/steam-patch.service"

# Run service
systemctl --user daemon-reload
systemctl --user enable steam-patch.service 2> /dev/null
systemctl --user start steam-patch.service 2> /dev/null

echo "Disabling Steam file protection..."

CONFIG_FILE="${HOME}/.config/environment.d/gamescope-session-plus.conf"
if [ ! -f "${CONFIG_FILE}" ]; then
  echo "File does not exist. Creating ${CONFIG_FILE}..."
  mkdir -p "$(dirname "${CONFIG_FILE}")" # Ensure parent directory exists
  touch "${CONFIG_FILE}"
fi

if grep -q "^CLIENTCMD" "${CONFIG_FILE}"; then
  echo "Steam command is already overwritten for the session"
else
  printf "# Disable steam file protection\n" >> "${CONFIG_FILE}"
  echo 'CLIENTCMD="steam -noverifyfiles -norepairfiles -gamepadui -steamos3 -steampal -steamdeck"' >> "${CONFIG_FILE}"
  echo "Steam file protection was disabled"
fi