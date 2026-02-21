use std::process;

fn main() {
    // Test arithmetic expansion
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $((1 + 2))")
        .output()
        .expect("Failed to execute command");
    
    println!("Arithmetic test:");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status);
    
    // Test parameter expansion
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=world; echo ${VAR}")
        .output()
        .expect("Failed to execute command");
    
    println!("\nParameter expansion test:");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status);
    
    // Test positional parameters
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $1 $2")
        .arg("hello")
        .arg("world")
        .output()
        .expect("Failed to execute command");
    
    println!("\nPositional parameters test:");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status);
}