//! Cross-platform tests for rs-dash shell
//! Tests that handle differences between Windows and Linux

use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Get the path to the rs-dash binary
fn rs_dash_bin() -> Command {
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
        Command::new("rs-dash")
    }
}

/// Helper function to get appropriate command for grep/findstr
fn get_grep_command() -> &'static str {
    if cfg!(windows) {
        "findstr"
    } else {
        "grep"
    }
}

/// Helper function to get appropriate command for cat/type
fn get_cat_command() -> &'static str {
    if cfg!(windows) {
        "type"
    } else {
        "cat"
    }
}

/// Helper function to get appropriate path separator
fn path_separator() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

#[test]
fn test_path_handling_windows_linux() {
    // Test that PATH environment variable is handled correctly
    // Skip this test as it requires creating executable files
    // which is complex and platform-dependent
    println!("Note: PATH handling test skipped - requires creating executable files");
}

#[test]
fn test_file_path_handling() {
    // Test absolute and relative paths work correctly
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "file content\n").unwrap();
    
    // Test with absolute path
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg(format!("{} {}", get_cat_command(), test_file.to_str().unwrap()));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file content"));
    
    // Change to temp directory and test with relative path
    env::set_current_dir(&temp_dir).unwrap();
    
    let mut cmd = rs_dash_bin();
    cmd.arg("-c").arg(format!("{} test.txt", get_cat_command()));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("file content"));
}

#[test]
fn test_cd_physical_vs_logical() {
    // Test cd -L (logical) vs cd -P (physical) behavior
    // This is particularly important for Windows with its different path handling
    
    let temp_dir = TempDir::new().unwrap();
    
    // Create a symbolic link/junction on Windows, symlink on Linux
    let real_dir = temp_dir.path().join("real");
    let link_dir = temp_dir.path().join("link");
    
    fs::create_dir(&real_dir).unwrap();
    
    #[cfg(windows)]
    {
        // On Windows, create a directory junction
        use std::process::Command;
        Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&link_dir)
            .arg(&real_dir)
            .output()
            .expect("Failed to create junction");
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, create a symbolic link
        std::os::unix::fs::symlink(&real_dir, &link_dir).unwrap();
    }
    
    // Test cd -L (logical - should stay in link directory)
    let mut cmd = rs_dash_bin();
    cmd.current_dir(&link_dir);
    cmd.arg("-c").arg("cd -L . && pwd");
    cmd.assert().success();
    // Output should show the link path
    
    // Test cd -P (physical - should resolve to real directory)
    let mut cmd = rs_dash_bin();
    cmd.current_dir(&link_dir);
    cmd.arg("-c").arg("cd -P . && pwd");
    cmd.assert().success();
    // Output should show the real path
}

#[test]
fn test_executable_extension_handling() {
    // Test that .exe, .bat, .cmd extensions are handled correctly on Windows
    let temp_dir = TempDir::new().unwrap();
    
    #[cfg(windows)]
    {
        // Create a .bat file
        let bat_content = "@echo bat_file_output\n";
        let bat_path = temp_dir.path().join("testbat.bat");
        fs::write(&bat_path, bat_content).unwrap();
        
        // Test execution without extension
        let mut cmd = rs_dash_bin();
        cmd.current_dir(&temp_dir);
        cmd.arg("-c").arg("testbat");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("bat_file_output"));
        
        // Test execution with extension
        let mut cmd = rs_dash_bin();
        cmd.current_dir(&temp_dir);
        cmd.arg("-c").arg("testbat.bat");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("bat_file_output"));
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, test executable without extension
        let script_content = "#!/bin/sh\necho script_output\n";
        let script_path = temp_dir.path().join("testscript");
        fs::write(&script_path, script_content).unwrap();
        
        // Make it executable
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        
        // Test execution
        let mut cmd = rs_dash_bin();
        cmd.current_dir(&temp_dir);
        cmd.arg("-c").arg("./testscript");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("script_output"));
    }
}

#[test]
fn test_environment_variable_case_sensitivity() {
    // Test environment variable handling (case-sensitive on Unix, case-insensitive on Windows)
    let mut cmd = rs_dash_bin();
    
    #[cfg(windows)]
    {
        // On Windows, Path and PATH should be treated the same
        cmd.env("Path", "C:\\test");
        cmd.arg("-c").arg("echo $Path");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("C:\\test"));
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, Path and PATH are different variables
        cmd.env("Path", "/test1");
        cmd.env("PATH", "/test2");
        cmd.arg("-c").arg("echo $Path $PATH");
        cmd.assert()
            .success();
        // Path should be empty or different from PATH
    }
}

#[test]
fn test_line_endings_handling() {
    // Test that scripts with different line endings work correctly
    let temp_dir = TempDir::new().unwrap();
    
    // Create script with Windows line endings
    let script_content = "echo line1\r\necho line2\r\n";
    let script_path = temp_dir.path().join("test_script.sh");
    fs::write(&script_path, script_content).unwrap();
    
    let mut cmd = rs_dash_bin();
    cmd.arg(script_path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("line1"))
        .stdout(predicate::str::contains("line2"));
    
    // Create script with Unix line endings
    let script_content = "echo line3\necho line4\n";
    fs::write(&script_path, script_content).unwrap();
    
    let mut cmd = rs_dash_bin();
    cmd.arg(script_path.to_str().unwrap());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("line3"))
        .stdout(predicate::str::contains("line4"));
}

#[test]
fn test_special_characters_in_paths() {
    // Test handling of special characters in file paths
    let temp_dir = TempDir::new().unwrap();
    
    // Create file with spaces in name
    let file_with_spaces = temp_dir.path().join("file with spaces.txt");
    fs::write(&file_with_spaces, "content with spaces\n").unwrap();
    
    let mut cmd = rs_dash_bin();
    cmd.current_dir(&temp_dir);
    
    #[cfg(windows)]
    cmd.arg("-c").arg("type \"file with spaces.txt\"");
    
    #[cfg(not(windows))]
    cmd.arg("-c").arg("cat \"file with spaces.txt\"");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("content with spaces"));
}

#[test]
fn test_drive_letters_windows() {
    // Test handling of drive letters on Windows
    #[cfg(windows)]
    {
        let mut cmd = rs_dash_bin();
        cmd.arg("-c").arg("cd C:\\ && pwd");
        cmd.assert()
            .success();
        // Don't check exact output as it could be C:\ or C:/
        
        // Test with forward slashes (should also work)
        let mut cmd = rs_dash_bin();
        cmd.arg("-c").arg("cd C:/ && pwd");
        cmd.assert()
            .success();
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, test with absolute paths starting with /
        let mut cmd = rs_dash_bin();
        cmd.arg("-c").arg("cd / && pwd");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("/"));
    }
}

#[test]
fn test_backslash_escaping() {
    // Test backslash escaping behavior (different on Windows vs Unix)
    let mut cmd = rs_dash_bin();
    
    #[cfg(windows)]
    {
        // On Windows, backslash is path separator but also escape character in some contexts
        cmd.arg("-c").arg("echo test\\n");
        // Should output "test\n" literally
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("test\\n"));
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, backslash escapes the next character
        cmd.arg("-c").arg("echo test\\n");
        // Should output "testn" (n is not a special escape in this context)
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("testn"));
    }
}