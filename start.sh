#!/bin/bash

cargo build --release || { echo "[SH BUILD ERROR] Cargo build failed!"; exit 1; }

API_KEY=$(grep "^TELEGRAM_TOKEN=" config.txt | cut -d'=' -f2 | tr -d '[:space:]')

if [ -z "$API_KEY" ]; then
    echo "[SH BOT TOKEN ERROR] Token not found"
    exit 1
fi

echo "Bot TOKEN found!"
export TELOXIDE_TOKEN=$API_KEY

./target/release/rust_geo_sniffer
