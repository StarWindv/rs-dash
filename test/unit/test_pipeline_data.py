#!/usr/bin/env python3
"""
Test pipeline data flow
"""

import sys
import os
import subprocess

def run_rs_dash(cmd):
    """Run a command through rs-dash and return output and exit code"""
    target_dir = os.path.join(os.path.dirname(__file__), "..", "..", "target", "debug")
    rs_dash_exe = "rs-dash.exe" if os.name == 'nt' else "rs-dash"
    rs_dash_path = os.path.join(target_dir, rs_dash_exe)
    
    result = subprocess.run([rs_dash_path, "-c", cmd], 
                          capture_output=True, text=True)
    
    return result.stdout.strip(), result.stderr.strip(), result.returncode

def test_pipeline_data_flow():
    """Test that data flows correctly through pipelines"""
    print("Testing pipeline data flow")
    print("=" * 60)
    
    # Create a simple test script that writes to stdout
    test_script = """
import sys
for i in range(3):
    print(f"line {i+1}")
sys.exit(0)
"""
    
    # Write test script
    script_path = os.path.join(os.path.dirname(__file__), "..", "..", "test_pipe_data.py")
    with open(script_path, "w") as f:
        f.write(test_script)
    
    tests = []
    
    # On Windows, we need to use python to run the script
    if os.name == 'nt':
        python_exe = sys.executable
        tests = [
            (f'"{python_exe}" "{script_path}" | "{python_exe}" -c "import sys; [print(line.strip()) for line in sys.stdin]"', 
             "line 1\nline 2\nline 3", 0, "Python to Python pipe"),
        ]
    else:
        tests = [
            ('seq 3 | cat', '1\n2\n3', 0, "seq to cat pipe"),
        ]
    
    passed = 0
    failed = 0
    
    for cmd, expected_output, expected_code, desc in tests:
        print(f"\nTest: {desc}")
        print(f"Command: {cmd}")
        
        output, stderr, code = run_rs_dash(cmd)
        
        success = True
        if code != expected_code:
            print(f"FAIL: Exit code {code} (expected {expected_code})")
            if stderr:
                print(f"Stderr: {stderr}")
            success = False
        
        # Normalize line endings and compare
        output_normalized = output.replace('\r\n', '\n').strip()
        expected_normalized = expected_output.replace('\r\n', '\n').strip()
        
        if output_normalized != expected_normalized:
            print(f"FAIL: Output '{output}' (expected '{expected_output}')")
            success = False
        
        if success:
            print(f"PASS")
            passed += 1
        else:
            failed += 1
    
    # Clean up
    if os.path.exists(script_path):
        os.remove(script_path)
    
    print(f"\n{'='*60}")
    print(f"Pipeline data flow tests: {passed} passed, {failed} failed")
    
    return failed == 0

if __name__ == "__main__":
    success = test_pipeline_data_flow()
    sys.exit(0 if success else 1)