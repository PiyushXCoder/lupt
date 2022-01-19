#!/bin/sh

set -e

echo => Starting lupt server
/app/bin/lupt --config-file /app/config.json --static_path /app/static --bind_address 0.0.0.0 --port 8080
