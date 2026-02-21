use std::process;

fn main() {
    // Debug positional parameters
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo Number of args: $#; echo Args: $@")
        .arg("arg1")
        .arg("arg2")
        .arg("arg3")
        .output()
        .expect("Failed to execute command");
    
    println!("Positional parameters debug:");
    println!("Command: rs-dash -c 'echo Number of args: $#; echo Args: $@' arg1 arg2 arg3");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status);
    
    // Test $1, $2 specifically
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo First: $1; echo Second: $2")
        .arg("first")
        .arg("second")
        .output()
        .expect("Failed to execute command");
    
    println!("\nSpecific positional parameters:");
    println!("Command: rs-dash -c 'echo First: $1; echo Second: $2' first second");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit code: {}", output.status);
}