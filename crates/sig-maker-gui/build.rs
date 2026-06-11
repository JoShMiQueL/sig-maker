fn main() {
    // Build the frontend before building the Tauri app
    #[cfg(target_os = "windows")]
    {
        let status = std::process::Command::new("cmd")
            .args(["/C", "cd frontend && npm run build"])
            .status()
            .expect("Failed to build frontend");
        if !status.success() {
            panic!("Frontend build failed");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let status = std::process::Command::new("sh")
            .args(["-c", "cd frontend && npm run build"])
            .status()
            .expect("Failed to build frontend");
        if !status.success() {
            panic!("Frontend build failed");
        }
    }

    tauri_build::build()
}
