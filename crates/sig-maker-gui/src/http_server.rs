//! Dev HTTP server — serves the frontend and exposes sig-maker-core as a REST API.
//!
//! Only compiled when the `dev-server` feature is enabled.
//! Allows browser-based development and Playwright MCP automation.
//!
//! Endpoints
//! ---------
//!   GET  /                → serves frontend/dist/index.html
//!   GET  /api/formats     → JSON array of {id, name}
//!   POST /api/convert     → body: {input, format_id} → ConversionResult

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sig_maker_core::formats::Format;
use sig_maker_core::{analyze_aobs, convert_to_format, parse_aobs, parse_single_pattern};

pub const PORT: u16 = 7331;

// ── Types ──────────────────────────────────────────────────────────────────────

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
}

// ── Entry point ────────────────────────────────────────────────────────────────

pub fn run() {
    let addr = format!("127.0.0.1:{PORT}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[dev-server] Could not bind to {addr}: {e}");
            return;
        }
    };
    eprintln!("[dev-server] Running → http://{addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => eprintln!("[dev-server] Connection error: {e}"),
        }
    }
}

// ── Connection handler ────────────────────────────────────────────────────────

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    let request_line = request_line.trim().to_string();

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

    let body = if content_length > 0 {
        let mut buf = vec![0u8; content_length];
        let _ = reader.read_exact(&mut buf);
        String::from_utf8_lossy(&buf).to_string()
    } else {
        String::new()
    };

    let mut parts = request_line.splitn(3, ' ');
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    let response = match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => serve_index(),
        ("GET", p) if p.starts_with("/_astro/") => serve_static(p),
        ("GET", "/api/formats") => handle_get_formats(),
        ("POST", "/api/convert") => handle_post_convert(&body),
        ("OPTIONS", _) => cors_preflight(),
        _ => not_found(),
    };

    let _ = stream.write_all(response.as_bytes());
}

// ── Route handlers ────────────────────────────────────────────────────────────

fn serve_index() -> String {
    let path = frontend_dist().join("index.html");
    match std::fs::read_to_string(&path) {
        Ok(contents) => html_response(&contents),
        Err(_) => not_found(),
    }
}

fn serve_static(url_path: &str) -> String {
    // Strip leading slash and resolve against frontend/dist
    let rel = url_path.trim_start_matches('/');
    let path = frontend_dist().join(rel);
    match std::fs::read(&path) {
        Ok(bytes) => {
            let mime = mime_from_path(&path);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n",
                bytes.len()
            ) + &String::from_utf8_lossy(&bytes)
        }
        Err(_) => not_found(),
    }
}

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

    let format = match Format::from_string(&req.format_id) {
        Some(f) => f,
        None => return json_error(400, &format!("Unknown format: {}", req.format_id)),
    };

    let non_empty_lines: Vec<&str> = req
        .input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("//"))
        .collect();

    let parsed = if non_empty_lines.len() >= 2 {
        let aobs = parse_aobs(&req.input);
        if aobs.len() < 2 {
            return json_error(
                422,
                "Could not parse multiple AOB instances — check the input format",
            );
        }
        let first_len = aobs[0].bytes.len();
        for (i, aob) in aobs.iter().enumerate() {
            if aob.bytes.len() != first_len {
                return json_error(
                    422,
                    &format!(
                        "Line {} has {} bytes, expected {}",
                        i + 1,
                        aob.bytes.len(),
                        first_len
                    ),
                );
            }
        }
        analyze_aobs(&req.input)
    } else {
        match parse_single_pattern(&req.input) {
            Some(p) => p,
            None => return json_error(422, "Could not parse pattern — check the input format"),
        }
    };

    let output = convert_to_format(&parsed, format);
    let byte_count = parsed.len();
    let wildcard_count = parsed
        .iter()
        .filter(|b| matches!(b, sig_maker_core::BytePattern::Wildcard))
        .count();

    let result = ConversionResult {
        output,
        byte_count,
        wildcard_count,
    };
    json_response(200, &serde_json::to_string(&result).unwrap())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn frontend_dist() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("frontend")
        .join("dist")
}

fn mime_from_path(path: &PathBuf) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("js") => "application/javascript",
        Some("css") => "text/css",
        Some("html") => "text/html; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    }
}

fn html_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}

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
