//! Test HTTP server — exposes sig-maker-core as a REST API so that
//! browser-based tooling (Playwright, etc.) can drive the UI without Tauri.
//!
//! Endpoints
//! ---------
//!   GET  /                        → serves frontend/index.html
//!   GET  /styles.css              → serves frontend/styles.css
//!   GET  /main.js                 → serves frontend/main.js
//!   GET  /api/formats             → JSON array of {id, name}
//!   POST /api/convert             → body: {input, format_id} → ConversionResult
//!
//! Usage: cargo run --bin sig-maker-testserver
//! Then open http://localhost:7331 in your browser or with Playwright.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sig_maker_core::formats::Format;
use sig_maker_core::{convert_to_format, get_pattern_stats, parse_single_pattern};

const PORT: u16 = 7331;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ConvertRequest {
    input: String,
    format_id: String,
}

#[derive(Debug, Serialize)]
struct ConversionResult {
    output: String,
    byte_count: usize,
    wildcard_count: usize,
    specificity: f64,
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let addr = format!("127.0.0.1:{PORT}");
    let listener = TcpListener::bind(&addr).unwrap_or_else(|e| {
        eprintln!("Could not bind to {addr}: {e}");
        std::process::exit(1);
    });
    println!("Test server running → http://{addr}");
    println!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => eprintln!("Connection error: {e}"),
        }
    }
}

// ── Connection handler ────────────────────────────────────────────────────────

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));

    // Read request line
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim().to_string();

    // Read headers (consume until blank line)
    let mut content_length: usize = 0;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).is_err() {
            break;
        }
        let header = header.trim();
        if header.is_empty() {
            break;
        }
        if let Some(val) = header.strip_prefix("Content-Length:") {
            content_length = val.trim().parse().unwrap_or(0);
        }
    }

    // Read body if present
    let body = if content_length > 0 {
        let mut buf = vec![0u8; content_length];
        use std::io::Read;
        let _ = reader.read_exact(&mut buf);
        String::from_utf8_lossy(&buf).to_string()
    } else {
        String::new()
    };

    // Parse method + path
    let mut parts = request_line.splitn(3, ' ');
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    // Route
    let response = match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => serve_file("index.html", "text/html"),
        ("GET", "/styles.css") => serve_file("styles.css", "text/css"),
        ("GET", "/main.js") => serve_file("main.js", "application/javascript"),
        ("GET", "/api/formats") => handle_get_formats(),
        ("POST", "/api/convert") => handle_post_convert(&body),
        ("OPTIONS", _) => cors_preflight(),
        _ => not_found(),
    };

    let _ = stream.write_all(response.as_bytes());
}

// ── Route handlers ────────────────────────────────────────────────────────────

fn handle_get_formats() -> String {
    let formats: Vec<_> = Format::all()
        .iter()
        .map(|f| {
            json!({
                "id": format!("{:?}", f).to_lowercase(),
                "name": f.name()
            })
        })
        .collect();
    json_response(200, &serde_json::to_string(&formats).unwrap())
}

fn handle_post_convert(body: &str) -> String {
    let req: ConvertRequest = match serde_json::from_str(body) {
        Ok(r) => r,
        Err(e) => return json_error(400, &format!("Bad request: {e}")),
    };

    let parsed = match parse_single_pattern(&req.input) {
        Some(p) => p,
        None => return json_error(422, "Could not parse pattern — check the input format"),
    };

    let format = match Format::from_string(&req.format_id) {
        Some(f) => f,
        None => return json_error(400, &format!("Unknown format: {}", req.format_id)),
    };

    let output = convert_to_format(&parsed, format);
    let stats = get_pattern_stats(&parsed);

    let result = ConversionResult {
        output,
        byte_count: stats.total_bytes(),
        wildcard_count: stats.full_wildcards(),
        specificity: stats.compression_ratio(),
    };
    json_response(200, &serde_json::to_string(&result).unwrap())
}

// ── Static file serving ───────────────────────────────────────────────────────

fn frontend_dir() -> PathBuf {
    // When running via `cargo run`, __FILE__ is in src/, frontend/ is sibling
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("frontend");
    p
}

fn serve_file(filename: &str, content_type: &str) -> String {
    let path = frontend_dir().join(filename);
    match std::fs::read_to_string(&path) {
        Ok(content) => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {content_type}; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        ),
        Err(_) => not_found(),
    }
}

// ── HTTP helpers ──────────────────────────────────────────────────────────────

fn json_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}

fn json_error(status: u16, msg: &str) -> String {
    let body = json!({ "error": msg }).to_string();
    format!(
        "HTTP/1.1 {status} Error\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}

fn cors_preflight() -> String {
    "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n".to_string()
}

fn not_found() -> String {
    "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
}
