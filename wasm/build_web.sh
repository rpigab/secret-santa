#!/bin/bash

cd "$(dirname "$0")"

rm -rf ./web/pkg

wasm-pack build --target web --out-dir ./web/pkg --no-typescript
