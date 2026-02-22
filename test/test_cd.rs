use std::process::Command;

fn main() {
    println!("Testing rs-dash cd command...");
    
    // Test 1: cd -L
    let output = Command::new("target/release/rs-dash.exe")
        .arg("-c")
        .arg("cd -L . && pwd")
        .output()
        .expect("Failed to execute command");
    
    println!("Test 1 - cd -L . && pwd:");
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // Test 2: cd -P
    let output = Command::new("target/release/rs-dash.exe")
        .arg("-c")
        .arg("cd -P . && pwd")
        .output()
        .expect("Failed to execute command");
    
    println!("\nTest 2 - cd -P . && pwd:");
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    // Test 3: pwd with options
    let output = Command::new("target/release/rs-dash.exe")
        .arg("-c")
        .arg("pwd && pwd -L && pwd -P")
        .output()
        .expect("Failed to execute command");
    
    println!("\nTest 3 - pwd && pwd -L && pwd -P:");
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
}