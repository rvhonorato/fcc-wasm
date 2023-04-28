#!/bin/bash

npm run build:wasm

npm install ./wasm-lib/pkg

npm run start
