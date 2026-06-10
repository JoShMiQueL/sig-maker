# Run the same checks as CI

Write-Host "==> Running cargo fmt --check..."
cargo fmt -- --check
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "==> Running cargo clippy..."
cargo clippy --workspace -- -D warnings
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "==> Running cargo build..."
cargo build --workspace --verbose
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "==> Running cargo test..."
cargo test --workspace --verbose
if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "==> All checks passed!"
