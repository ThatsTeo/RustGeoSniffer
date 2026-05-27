#!/bin/bash

pkill -9 rust_geo_sniffe || { echo "[SH STOP ERROR] Process not temrinated!"; exit 1; }

echo "Process terminated!"
