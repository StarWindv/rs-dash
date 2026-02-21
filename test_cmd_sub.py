#!/usr/bin/env python3
"""
Test command substitution specifically
"""

import subprocess
import os

def test_cmd_sub():
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    tests = [
        ("echo $(echo test)", "Simple command substitution"),
        ("echo $(pwd)", "Command substitution with pwd"),
        ("echo before $(echo middle) after", "Command substitution in middle"),
        ("echo $(echo one; echo two)", "Multi-line command substitution"),
    ]
    
    for cmd, desc in tests:
        print(f"\n{desc}:")
        print(f"  Command: {cmd}")
        result = subprocess.run([rs_dash_path, "-c", cmd],
                               capture_output=True, text=True, timeout=2)
        print(f"  Exit: {result.returncode}")
        print(f"  Output: {repr(result.stdout.strip())}")
        if result.stderr:
            print(f"  Error: {result.stderr.strip()}")

if __name__ == "__main__":
    test_cmd_sub()