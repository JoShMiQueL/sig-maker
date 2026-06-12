#!/bin/bash
cd crates/sig-maker-gui/frontend && npm run build
cd ../..
cargo tauri build
