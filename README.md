# boat_telemetry

## Setup

```sh
# Create the boat-telemetry APT source list
sudo tee /etc/apt/sources.list.d/boat-telemetry.list > /dev/null << EOF
deb [trusted=yes] https://ninefx.github.io/boat_telemetry bullseye main
EOF

# Create the boat_telemetry logrotate config
sudo tee /etc/logrotate.d/boat_telemetry > /dev/null << 'EOF'
/var/log/boat_telemetry.log {
    maxsize 10M
    weekly
    rotate 4
    compress
    notifempty
    missingok
    create 0644 root root
}
EOF
```