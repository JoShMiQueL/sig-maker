# sig-maker-gui Documentation Guidelines

This crate provides a Tauri GUI for sig-maker-core with an Astro frontend.

## Documentation Standards

When documenting sig-maker-gui:
- Focus on the Tauri backend architecture and Astro frontend
- Document the dual-mode communication (Tauri IPC vs HTTP fallback)
- Include setup instructions for development
- Document the dev-server feature for Playwright MCP testing
- Explain the build and release process

## Key Files to Document

### Backend (Rust)
- `src/main.rs` - Tauri entry point and command registration
- `src/commands.rs` - Tauri commands exposed to the frontend
- `src/http_server.rs` - HTTP server for development (only with `dev-server` feature)
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
- Dev server: HTTP server at localhost:7331 (only with `dev-server` feature)

## Development Workflow

**Development (browser + Playwright MCP):**
```bash
# From workspace root:
.\scripts\dev.bat      # Windows
./scripts/dev.sh       # Linux/macOS

# Then open browser or use Playwright MCP at http://localhost:7331
```

The `dev-server` feature starts an HTTP server inside the Tauri process that:
- Serves `frontend/dist/` (the compiled Astro output)
- Exposes `GET /api/formats` and `POST /api/convert` backed by sig-maker-core
- Allows Playwright MCP to automate the GUI from the browser

**Production:**
```bash
# From workspace root:
.\scripts\build.bat    # Windows
./scripts/build.sh     # Linux/macOS
```

## Build Process

- Frontend build: `cd frontend && npm run build` → output in `frontend/dist/`
- Tauri reads `frontend/dist/` via `frontendDist` in `tauri.conf.json`
- The `dev-server` feature is NOT included in production builds (`cargo tauri build`)

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
- Cargo.toml: dev-server feature flag for HTTP server
