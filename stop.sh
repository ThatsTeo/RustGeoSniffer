#!/bin/bash

pkill -9 rust_geo_sniffe || { echo "[SH STOP ERROR] Process not terminated!"; exit 1; }
tmux kill-session -t geo_sniffer || { echo "[SH TMUX ERROR] Session not terminated!"; exit 1; }

echo "Exit completed!"
