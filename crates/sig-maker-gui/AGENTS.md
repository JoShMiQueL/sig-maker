# sig-maker-gui Documentation Guidelines

This crate provides a Tauri GUI for sig-maker-core with an Astro frontend.

## Documentation Standards

When documenting sig-maker-gui:
- Focus on the Tauri backend architecture and Astro frontend
- Document the dual-mode communication (Tauri IPC vs HTTP fallback)
- Include setup instructions for development
- Document the testserver for browser-based testing
- Explain the build and release process

## Key Files to Document

### Backend (Rust)
- `src/main.rs` - Tauri entry point and command registration
- `src/commands.rs` - Tauri commands exposed to the frontend
- `src/testserver.rs` - HTTP test server for browser-based testing
- `tauri.conf.json` - Tauri configuration

### Frontend (Astro)
- `frontend/src/pages/index.astro` - Main page component
- `frontend/src/styles/global.css` - Global styles
- `frontend/src/scripts/main.js` - Frontend logic and backend abstraction
- `frontend/astro.config.mjs` - Astro configuration
- `frontend/package.json` - Frontend dependencies

## Architecture

Document:
- How Tauri IPC bridge works (window.__TAURI__.core.invoke)
- How HTTP fallback works (fetch to /api/* endpoints)
- The invoke() abstraction layer that switches between modes
- Tauri commands: get_formats, convert_pattern

## Development Workflow

Document:
- How to run Astro dev server: `cd frontend && npm run dev`
- How to run Tauri dev: `cargo tauri dev` (requires tauri-cli)
- How to run testserver: `cargo run --bin sig-maker-testserver`
- How the testserver serves from frontend/dist (Astro build) or frontend/ (vanilla fallback)

## Build Process

Document:
- How to build the Astro frontend: `cd frontend && npm run build`
- How the build output goes to frontend/dist
- How Tauri uses frontendDist in production
- How the testserver prefers frontend/dist over frontend/

## Frontend Features

Document:
- Auto-convert on input (debounced, 300ms)
- Format toggle buttons (all selected by default)
- Multiple output cards (one per selected format)
- Drag & drop file support
- Copy button per output card
- Error handling and display

## Configuration

Document:
- tauri.conf.json: devUrl for development, frontendDist for production
- astro.config.mjs: port 4321, outDir dist
- How the configuration enables hybrid development (Astro dev server + Tauri build)
