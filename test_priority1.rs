use std::process;

fn main() {
    println!("=== Testing Priority 1 Features ===\n");
    
    // Test 1: Basic parameter expansion
    println!("1. Basic parameter expansion:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=hello; echo ${VAR}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=hello; echo ${{VAR}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 2: Default value expansion
    println!("2. Default value expansion:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo ${UNDEF:-default}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo ${{UNDEF:-default}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 3: Assign default value
    println!("3. Assign default value:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo ${UNDEF:=assigned}; echo $UNDEF")
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo ${{UNDEF:=assigned}}; echo $UNDEF");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 4: String length
    println!("4. String length:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=hello; echo ${#VAR}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=hello; echo ${{#VAR}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 5: Arithmetic expansion
    println!("5. Arithmetic expansion:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $(( (1 + 2) * 3 ))")
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo $(( (1 + 2) * 3 ))");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 6: Bitwise operations
    println!("6. Bitwise operations:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $(( 5 & 3 ))")  // 0101 & 0011 = 0001
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo $(( 5 & 3 ))");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 7: Comparison operations
    println!("7. Comparison operations:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $(( 2 > 1 ))")  // true = 1
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo $(( 2 > 1 ))");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 8: Ternary operator
    println!("8. Ternary operator:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("echo $(( 1 ? 100 : 200 ))")
        .output()
        .expect("Failed to execute command");
    println!("   Command: echo $(( 1 ? 100 : 200 ))");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 9: Pattern removal (suffix)
    println!("9. Pattern removal (suffix):");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=filename.txt; echo ${VAR%.txt}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=filename.txt; echo ${{VAR%.txt}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 10: Pattern removal (prefix)
    println!("10. Pattern removal (prefix):");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=filename.txt; echo ${VAR#file}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=filename.txt; echo ${{VAR#file}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 11: Pattern substitution
    println!("11. Pattern substitution:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=hello world; echo ${VAR/world/everyone}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=hello world; echo ${{VAR/world/everyone}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}\n", output.status);
    
    // Test 12: Case modification
    println!("12. Case modification:");
    let output = process::Command::new("target/debug/rs-dash")
        .arg("-c")
        .arg("VAR=hello; echo ${VAR^^}")
        .output()
        .expect("Failed to execute command");
    println!("   Command: VAR=hello; echo ${{VAR^^}}");
    println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
    println!("   Status: {}", output.status);
}