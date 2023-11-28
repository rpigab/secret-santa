#!/bin/bash

set -e

export NVS_HOME="$HOME/.nvs"
[ -s "$NVS_HOME/nvs.sh" ] && . "$NVS_HOME/nvs.sh"

wasm-pack build
cd web
nvs auto
npm ci
npm run start
