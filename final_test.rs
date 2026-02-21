use std::process;

fn main() {
    println!("=== Final Priority 1 Test ===\n");
    
    let tests = vec![
        // Parameter expansion tests
        ("Basic parameter", "VAR=hello; echo ${VAR}", "hello"),
        ("Default value", "echo ${UNDEF:-default}", "default"),
        ("Assign default", "echo ${UNDEF:=assigned}; echo $UNDEF", "assigned\nassigned"),
        ("String length", "VAR=hello; echo ${#VAR}", "5"),
        ("Pattern suffix", "VAR=filename.txt; echo ${VAR%.txt}", "filename"),
        ("Pattern prefix", "VAR=filename.txt; echo ${VAR#file}", "name.txt"),
        ("Pattern substitution", "VAR=hello world; echo ${VAR/world/everyone}", "hello everyone"),
        ("Case upper", "VAR=hello; echo ${VAR^^}", "HELLO"),
        ("Case lower", "VAR=HELLO; echo ${VAR,,}", "hello"),
        
        // Arithmetic expansion tests
        ("Arithmetic basic", "echo $((1 + 2))", "3"),
        ("Arithmetic precedence", "echo $((1 + 2 * 3))", "7"),
        ("Arithmetic bitwise", "echo $((5 & 3))", "1"),
        ("Arithmetic comparison", "echo $((2 > 1))", "1"),
        ("Arithmetic ternary", "echo $((1 ? 100 : 200))", "100"),
        
        // Positional parameters tests
        ("Positional count", "echo $#", "3"),
        ("Positional first", "echo $1", "arg1"),
        ("Positional second", "echo $2", "arg2"),
        ("Positional all", "echo $@", "arg1 arg2 arg3"),
        ("Shell name", "echo $0", "rs-dash"),
        
        // Special parameters tests
        ("Exit status success", "true; echo $?", "0"),
        ("Exit status failure", "false; echo $?", "1"),
        ("PID", "echo $$", ""), // Just check it doesn't crash
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for (name, cmd, expected) in tests {
        let output = process::Command::new("target/debug/rs-dash")
            .arg("-c")
            .arg(cmd)
            .arg("arg1")
            .arg("arg2")
            .arg("arg3")
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                
                // For PID test, we just check it doesn't crash
                let success = if name == "PID" {
                    output.status.success() && stderr.is_empty()
                } else {
                    stdout == expected && output.status.success()
                };
                
                if success {
                    println!("✓ {}: PASS", name);
                    passed += 1;
                } else {
                    println!("✗ {}: FAIL", name);
                    println!("  Command: {}", cmd);
                    println!("  Expected: {}", expected);
                    println!("  Got: {}", stdout);
                    if !stderr.is_empty() {
                        println!("  Stderr: {}", stderr);
                    }
                    failed += 1;
                }
            }
            Err(e) => {
                println!("✗ {}: ERROR - {}", name, e);
                failed += 1;
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("Total tests: {}", passed + failed);
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    
    if failed == 0 {
        println!("\nAll Priority 1 tests passed! ✓");
    } else {
        println!("\nSome tests failed.");
        std::process::exit(1);
    }
}