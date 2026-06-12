@echo off
cd crates\sig-maker-gui\frontend
call npm run build
cd ..\..
cargo tauri build
