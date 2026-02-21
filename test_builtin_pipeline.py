#!/usr/bin/env python3
"""
Test builtin commands in pipeline
"""

import subprocess
import os

def test_builtin_pipeline():
    """Test builtin commands in pipeline"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    tests = [
        ("echo hello | echo world", "Test echo pipeline"),
        ("echo test1; echo test2 | echo test3", "Test mixed commands"),
        ("true | echo success", "Test true in pipeline"),
        ("false | echo failure", "Test false in pipeline"),
    ]
    
    for cmd, desc in tests:
        print(f"\nTest: {desc}")
        print(f"Command: {cmd}")
        result = subprocess.run([rs_dash_path, "-c", cmd],
                               capture_output=True, text=True, timeout=2)
        print(f"Exit code: {result.returncode}")
        print(f"Stdout: {result.stdout.strip()}")
        if result.stderr:
            print(f"Stderr: {result.stderr.strip()}")

if __name__ == "__main__":
    print("Testing builtin commands in pipeline")
    test_builtin_pipeline()