use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    // Test 1: Simple pipe with cmd commands
    println!("Test 1: echo hello | findstr hello");
    
    let mut echo_cmd = Command::new("cmd");
    echo_cmd.args(["/c", "echo hello"]);
    echo_cmd.stdout(Stdio::piped());
    
    let echo_child = echo_cmd.spawn().expect("Failed to spawn echo");
    
    let mut findstr_cmd = Command::new("cmd");
    findstr_cmd.args(["/c", "findstr hello"]);
    findstr_cmd.stdin(Stdio::from(echo_child.stdout.unwrap()));
    
    let output = findstr_cmd.output().expect("Failed to execute findstr");
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    
    // Test 2: Pipe chain
    println!("\nTest 2: Testing pipe chain");
    
    let mut first = Command::new("cmd");
    first.args(["/c", "echo test pipe"]);
    first.stdout(Stdio::piped());
    
    let mut second = Command::new("cmd");
    second.args(["/c", "findstr test"]);
    second.stdin(Stdio::piped());
    
    let first_child = first.spawn().expect("Failed to spawn first");
    let mut second_child = second.spawn().expect("Failed to spawn second");
    
    // Connect the pipes
    let first_output = first_child.wait_with_output().expect("Failed to wait for first");
    if let Some(mut second_stdin) = second_child.stdin.take() {
        second_stdin.write_all(&first_output.stdout).expect("Failed to write to stdin");
    }
    
    let second_output = second_child.wait_with_output().expect("Failed to wait for second");
    println!("Output: {}", String::from_utf8_lossy(&second_output.stdout));
}