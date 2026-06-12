fn main() {
    // Watch the frontend dist directory for changes to invalidate cache
    println!("cargo:rerun-if-changed=frontend/dist");
    tauri_build::build()
}
