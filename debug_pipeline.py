#!/usr/bin/env python3
"""
Simple test to debug pipeline issue
"""

import subprocess
import os

def run_test(cmd):
    """Run a test command"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    print(f"\nCommand: {cmd}")
    result = subprocess.run([rs_dash_path, "-c", cmd],
                           capture_output=True, text=True, timeout=2)
    print(f"Exit code: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    if result.stderr:
        print(f"Stderr: {result.stderr}")

if __name__ == "__main__":
    print("Debugging pipeline issue")
    
    # Test various pipeline scenarios
    tests = [
        "echo test",
        "echo hello | echo world",
        "echo first; echo second | echo third",
        "echo start && echo middle | echo end",
        "false || echo alternative | echo result",
    ]
    
    for test in tests:
        run_test(test)