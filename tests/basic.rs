//! Basic tests for rs-dash shell
//! Tests fundamental functionality and built-in commands

use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use std::path::Path;

/// Get the path to the rs-dash binary
fn rs_dash_bin() -> Command {
    // When running tests with `cargo test`, the binary is in target/debug/deps
    // When running integration tests, it's in target/debug
    // We'll try both paths
    let debug_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("rs-dash");
    
    let deps_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("deps")
        .join("rs-dash");
    
    if debug_path.exists() {
        Command::new(debug_path)
    } else if deps_path.exists() {
        Command::new(deps_path)
    } else {
        // Fallback to just "rs-dash" which should work if it's in PATH
        Command::new("rs-dash")
    }
}

#[test]
fn test_echo_basic() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo hello");
    cmd.assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn test_echo_multiple_args() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo hello world");
    cmd.assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_echo_empty() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo");
    cmd.assert()
        .success()
        .stdout("\n");
}

#[test]
fn test_true_command() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("true");
    cmd.assert()
        .success()
        .stdout("");
}

#[test]
fn test_false_command() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("false");
    cmd.assert()
        .failure()
        .stdout("");
}

#[test]
fn test_exit_zero() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("exit 0");
    cmd.assert()
        .success()
        .stdout("");
}

#[test]
fn test_exit_non_zero() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("exit 42");
    cmd.assert()
        .code(42)
        .stdout("");
}

#[test]
fn test_exit_no_arg() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("exit");
    cmd.assert()
        .success()
        .stdout("");
}

#[test]
fn test_pwd_command() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("pwd");
    cmd.assert()
        .success();
    // Don't check exact output as it depends on current directory
}

#[test]
fn test_help_command() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rs-dash v0.0.1"))
        .stdout(predicate::str::contains("Built-in commands"));
}

#[test]
fn test_version_flag() {
    let mut cmd = rs_dash_bin();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rs-dash version 0.0.1"));
}

#[test]
fn test_help_flag() {
    let mut cmd = rs_dash_bin();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rs-dash v0.0.1"));
}

#[test]
fn test_command_separator_semicolon() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo first; echo second");
    cmd.assert()
        .success()
        .stdout("first\nsecond\n");
}

#[test]
fn test_command_separator_and() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("true && echo success");
    cmd.assert()
        .success()
        .stdout("success\n");
}

#[test]
fn test_command_separator_or() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("false || echo fallback");
    cmd.assert()
        .success()
        .stdout("fallback\n");
}

#[test]
fn test_variable_assignment() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("VAR=test && echo $VAR");
    cmd.assert()
        .success()
        .stdout("test\n");
}

#[test]
fn test_exit_status_variable() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("true; echo $?");
    cmd.assert()
        .success()
        .stdout("0\n");
    
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("false; echo $?");
    cmd.assert()
        .success()
        .stdout("1\n");
}

#[test]
fn test_shell_pid_variable() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo $$");
    cmd.assert()
        .success();
    // Output should be a number
}

#[test]
fn test_shell_name_variable() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo $0");
    cmd.assert()
        .success()
        .stdout("rs-dash\n");
}

#[test]
fn test_command_substitution() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo $(echo test)");
    cmd.assert()
        .success()
        .stdout("test\n");
}

#[test]
fn test_command_substitution_multiple() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo $(echo hello) $(echo world)");
    cmd.assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_command_substitution_in_middle() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo prefix$(echo middle)suffix");
    cmd.assert()
        .success()
        .stdout("prefixmiddlesuffix\n");
}

#[test]
fn test_nested_command_substitution() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("echo $(echo $(echo nested))");
    cmd.assert()
        .success()
        .stdout("nested\n");
}

#[test]
fn test_cd_basic() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("cd . && pwd");
    cmd.assert()
        .success();
}

#[test]
fn test_cd_home() {
    // This test depends on HOME environment variable
    // Try to get HOME or USERPROFILE
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok();
    
    if home_dir.is_none() || home_dir.as_ref().unwrap().trim().is_empty() {
        return; // Skip test if neither is set
    }
    
    let mut cmd = rs_dash_bin();
    // Set HOME environment variable for the command
    cmd.env("HOME", home_dir.unwrap());
    cmd.arg("-c").arg("cd && pwd");
    cmd.assert()
        .success();
}

#[test]
fn test_multiple_variable_assignments() {
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg("A=1 B=2 && echo $A $B");
    cmd.assert()
        .success()
        .stdout("1 2\n");
}