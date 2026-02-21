#!/usr/bin/env python3
"""
Final demonstration of rs-dash capabilities
"""

import subprocess
import os
import tempfile

def run_demo():
    print("=" * 60)
    print("rs-dash Final Demonstration")
    print("=" * 60)
    
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "release", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    demos = [
        ("Basic echo", "echo 'Hello, rs-dash!'"),
        ("Multiple commands", "echo First; echo Second; echo Third"),
        ("Conditional execution (AND)", "echo Success && echo Continued"),
        ("Conditional execution (OR)", "false || echo 'This will print'"),
        ("Exit codes", "true; echo $?; false; echo $?"),
        ("Variable assignment", "GREETING='Hello World' && echo $GREETING"),
        ("Pipeline example", "echo 'line1\nline2\nline3' | grep 'line2'"),
        ("Help command", "help"),
    ]
    
    for name, command in demos:
        print(f"\n{'='*40}")
        print(f"Demo: {name}")
        print(f"Command: {command}")
        print(f"{'='*40}")
        
        try:
            result = subprocess.run(
                [rs_dash_path, "-c", command],
                capture_output=True,
                text=True,
                encoding='utf-8',
                errors='ignore'
            )
            
            if result.stdout:
                print("Output:")
                print(result.stdout.strip())
            
            if result.stderr:
                print("Errors:")
                print(result.stderr.strip())
                
            print(f"Exit code: {result.returncode}")
            
        except Exception as e:
            print(f"Error: {e}")
    
    # Test with a script file
    print(f"\n{'='*40}")
    print("Demo: Script File Execution")
    print(f"{'='*40}")
    
    script_content = """#!/bin/sh
# Test script for rs-dash
echo "Starting script..."
echo "Current directory: $(pwd)"
echo "Setting variable..."
MY_VAR="Script Variable"
echo "Variable value: $MY_VAR"
echo "Testing conditional..."
test -f README.md && echo "README exists" || echo "README not found"
echo "Script completed successfully"
exit 0
"""
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.sh', delete=False) as f:
        f.write(script_content)
        script_file = f.name
    
    try:
        print(f"Script content:\n{script_content}")
        print("\nExecuting script...")
        
        result = subprocess.run(
            [rs_dash_path, script_file],
            capture_output=True,
            text=True,
            encoding='utf-8',
            errors='ignore'
        )
        
        print("Script output:")
        print(result.stdout.strip())
        
        if result.stderr:
            print("Script errors:")
            print(result.stderr.strip())
            
        print(f"Script exit code: {result.returncode}")
        
    finally:
        os.unlink(script_file)
    
    print(f"\n{'='*60}")
    print("rs-dash Demonstration Complete!")
    print("=" * 60)

if __name__ == "__main__":
    run_demo()