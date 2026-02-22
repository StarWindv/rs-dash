use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing rs-dash cd command ===");
    
    // Build rs-dash if not already built
    println!("Building rs-dash...");
    let build_status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(".")
        .status()?;
    
    if !build_status.success() {
        eprintln!("Failed to build rs-dash");
        return Ok(());
    }
    
    let rs_dash_path = Path::new("target/release/rs-dash");
    if !rs_dash_path.exists() {
        eprintln!("rs-dash binary not found");
        return Ok(());
    }
    
    // Create test directory
    let test_dir = Path::new("test_cd_rs");
    if test_dir.exists() {
        fs::remove_dir_all(test_dir)?;
    }
    fs::create_dir_all(test_dir)?;
    
    // Create subdirectories
    let real_dir = test_dir.join("real");
    let sub_dir = real_dir.join("sub");
    fs::create_dir_all(&sub_dir)?;
    
    println!("Created test directory: {:?}", test_dir);
    println!("  real: {:?}", real_dir);
    println!("  sub: {:?}", sub_dir);
    
    // Test cases
    let test_cases = vec![
        // (test_name, command, expected_output_contains, should_succeed)
        ("cd without args (home)", "cd", "", true),
        ("cd with relative path", "cd test_cd_rs/real", "test_cd_rs/real", true),
        ("cd ..", "cd test_cd_rs/real && cd ..", "test_cd_rs", true),
        ("cd - (previous dir)", "cd test_cd_rs/real && cd -", "test_cd_rs", true),
        ("cd -L", "cd -L test_cd_rs/real", "test_cd_rs/real", true),
        ("cd -P", "cd -P test_cd_rs/real", "test_cd_rs/real", true),
        ("cd with invalid dir", "cd nonexistent_dir", "No such file", false),
        ("cd too many args", "cd a b c", "too many arguments", false),
        ("pwd -L", "cd test_cd_rs/real && pwd -L", "test_cd_rs/real", true),
        ("pwd -P", "cd test_cd_rs/real && pwd -P", "test_cd_rs/real", true),
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for (test_name, command, expected_contains, should_succeed) in test_cases {
        print!("Testing {}... ", test_name);
        
        let output = Command::new(rs_dash_path)
            .arg("-c")
            .arg(command)
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let success = output.status.success();
        
        let mut test_passed = false;
        
        if should_succeed {
            if success && (expected_contains.is_empty() || stdout.contains(expected_contains) || stderr.contains(expected_contains)) {
                test_passed = true;
            }
        } else {
            if !success && (expected_contains.is_empty() || stderr.contains(expected_contains)) {
                test_passed = true;
            }
        }
        
        if test_passed {
            println!("✓");
            passed += 1;
        } else {
            println!("✗");
            println!("  Command: {}", command);
            println!("  Expected success: {}", should_succeed);
            println!("  Actual success: {}", success);
            println!("  Expected contains: '{}'", expected_contains);
            println!("  Stdout: {}", stdout.trim());
            println!("  Stderr: {}", stderr.trim());
            failed += 1;
        }
    }
    
    // Cleanup
    fs::remove_dir_all(test_dir)?;
    
    println!("\n=== Test Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    
    if failed == 0 {
        println!("All tests passed!");
    } else {
        println!("Some tests failed!");
    }
    
    Ok(())
}