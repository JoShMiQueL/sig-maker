//! Integration tests for sig-maker CLI

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn run_sig_maker(args: &[&str]) -> (String, String, i32) {
    let output = Command::new("target/release/sig-maker.exe")
        .args(args)
        .output()
        .expect("Failed to run sig-maker");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    if !stdout.is_empty() {
        println!("STDOUT:\n{}", stdout);
    }
    if !stderr.is_empty() {
        println!("STDERR:\n{}", stderr);
    }

    (stdout, stderr, exit_code)
}

fn create_test_file(name: &str, content: &str) -> PathBuf {
    let path = PathBuf::from(format!("tests/{}.txt", name));
    fs::write(&path, content).expect("Failed to write test file");
    path
}

#[test]
fn test_version_flag() {
    let (output, _stderr, exit_code) = run_sig_maker(&["-v"]);
    assert!(exit_code == 0);
    assert!(output.contains("sig-maker"));
}

#[test]
fn test_pattern_validation_valid() {
    let path = create_test_file("valid_pattern", "0? 00 00 00 01 00 00 00 E? FF FF FF 00");
    let (output, _stderr, exit_code) = run_sig_maker(&["-c", path.to_str().unwrap()]);
    assert!(exit_code == 0);
    assert!(output.contains("Pattern is valid"));
    fs::remove_file(path).ok();
}

#[test]
fn test_pattern_validation_invalid() {
    let path = create_test_file("invalid_pattern", "XX YY ZZ");
    let (_output, stderr, exit_code) = run_sig_maker(&["-c", path.to_str().unwrap()]);
    assert!(exit_code == 1);
    assert!(stderr.contains("Pattern is invalid"));
    fs::remove_file(path).ok();
}

#[test]
fn test_quiet_mode() {
    let path = create_test_file("quiet_test", "0? 00 00 00 01 00 00 00 E? FF FF FF 00");
    let (output, _stderr, exit_code) = run_sig_maker(&["-q", path.to_str().unwrap()]);
    assert!(exit_code == 0);
    // Quiet mode should not show headers
    assert!(!output.contains("Sig-Maker"));
    assert!(!output.contains("=============================================="));
    fs::remove_file(path).ok();
}

#[test]
fn test_output_to_file() {
    let input_path = create_test_file("output_input", "0? 00 00 00 01 00 00 00 E? FF FF FF 00");
    let output_path = PathBuf::from("tests/output.txt");

    let (_output, _stderr, exit_code) = run_sig_maker(&[
        "-o",
        output_path.to_str().unwrap(),
        input_path.to_str().unwrap(),
    ]);
    assert!(exit_code == 0);

    // Check that file was created
    assert!(output_path.exists());

    // Check that file contains pattern
    let content = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(content.contains("0? 00 00 00 01 00 00 00 E? FF FF FF 00"));

    fs::remove_file(input_path).ok();
    fs::remove_file(output_path).ok();
}

#[test]
fn test_verbose_mode() {
    let path = create_test_file("verbose_test", "0? 00 00 00 01 00 00 00 E? FF FF FF 00");
    let (output, _stderr, exit_code) = run_sig_maker(&["--verbose", path.to_str().unwrap()]);
    assert!(exit_code == 0);
    // Verbose mode should show entropy and compression ratio
    assert!(output.contains("Entropy"));
    assert!(output.contains("Compression ratio"));
    fs::remove_file(path).ok();
}

#[test]
fn test_single_format_conversion() {
    let path = create_test_file("single_format", "0? 00 00 00 01 00 00 00 E? FF FF FF 00");
    let (output, _stderr, exit_code) = run_sig_maker(&["--to", "ce", path.to_str().unwrap()]);
    assert!(exit_code == 0);
    assert!(output.contains("0? 00 00 00 01 00 00 00 E? FF FF FF 00"));
    // Should not show all formats
    assert!(!output.contains("C++"));
    fs::remove_file(path).ok();
}
