@echo off
cd crates\sig-maker-gui\frontend
call npm run build
cd ..\..
cargo run --bin sig-maker-gui --features dev-server
