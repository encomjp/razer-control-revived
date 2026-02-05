#!/bin/bash
# Wrapper script to run daemon and GUI together in flatpak

# Start the daemon in background
/app/bin/razer-daemon &
DAEMON_PID=$!

# Give daemon time to create socket
sleep 0.5

# Run the GUI
/app/bin/razer-settings "$@"
EXIT_CODE=$?

# Clean up daemon when GUI exits
kill $DAEMON_PID 2>/dev/null

exit $EXIT_CODE
