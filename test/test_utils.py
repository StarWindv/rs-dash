#!/usr/bin/env python3
"""
Test utilities for rs-dash testing
"""

import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

class TestRunner:
    """Test runner for rs-dash"""
    
    def __init__(self, verbose=False):
        self.verbose = verbose
        self.project_root = Path(__file__).parent.parent
        self.rs_dash_path = self._find_rs_dash()
        self.dash_available = self._check_dash_available()
        
    def _find_rs_dash(self):
        """Find rs-dash executable"""
        # Try debug build first
        debug_path = self.project_root / "target" / "debug" / "rs-dash"
        if os.name == 'nt':
            debug_path = debug_path.with_suffix('.exe')
        
        if debug_path.exists():
            return debug_path
        
        # Try release build
        release_path = self.project_root / "target" / "release" / "rs-dash"
        if os.name == 'nt':
            release_path = release_path.with_suffix('.exe')
        
        if release_path.exists():
            return release_path
        
        raise FileNotFoundError(f"rs-dash executable not found. Build with 'cargo build' first.")
    
    def _check_dash_available(self):
        """Check if native dash is available for comparison"""
        try:
            result = subprocess.run(["dash", "-c", "echo test"], 
                                  capture_output=True, text=True)
            return result.returncode == 0
        except (FileNotFoundError, OSError):
            return False
    
    def run_command(self, shell_type, cmd, timeout=5, capture_output=True):
        """Run a command in specified shell"""
        if shell_type == "rs-dash":
            return self._run_rs_dash(cmd, timeout, capture_output)
        elif shell_type == "dash":
            return self._run_dash(cmd, timeout, capture_output)
        else:
            raise ValueError(f"Unknown shell type: {shell_type}")
    
    def _run_rs_dash(self, cmd, timeout, capture_output):
        """Run command in rs-dash"""
        args = [str(self.rs_dash_path), "-c", cmd]
        
        try:
            result = subprocess.run(args,
                                  capture_output=capture_output,
                                  text=True,
                                  encoding='utf-8',
                                  errors='ignore',
                                  timeout=timeout)
            return result
        except subprocess.TimeoutExpired:
            # Return a mock result for timeout
            class TimeoutResult:
                def __init__(self):
                    self.returncode = -1
                    self.stdout = ""
                    self.stderr = "TIMEOUT"
            return TimeoutResult()
    
    def _run_dash(self, cmd, timeout, capture_output):
        """Run command in native dash"""
        try:
            result = subprocess.run(["dash", "-c", cmd],
                                  capture_output=capture_output,
                                  text=True,
                                  encoding='utf-8',
                                  errors='ignore',
                                  timeout=timeout)
            return result
        except subprocess.TimeoutExpired:
            # Return a mock result for timeout
            class TimeoutResult:
                def __init__(self):
                    self.returncode = -1
                    self.stdout = ""
                    self.stderr = "TIMEOUT"
            return TimeoutResult()
    
    def compare_with_dash(self, test_name, cmd, skip_if_no_dash=True):
        """Compare rs-dash output with native dash"""
        if self.verbose:
            print(f"\n=== Test: {test_name} ===")
            print(f"Command: {cmd}")
        
        # Run with rs-dash
        rs_result = self.run_command("rs-dash", cmd)
        
        if self.verbose:
            print(f"rs-dash exit code: {rs_result.returncode}")
            print(f"rs-dash stdout: {rs_result.stdout.strip()}")
            if rs_result.stderr:
                print(f"rs-dash stderr: {rs_result.stderr.strip()}")
        
        # Try to run with dash if available
        dash_result = None
        if self.dash_available:
            dash_result = self.run_command("dash", cmd)
            
            if self.verbose:
                print(f"dash exit code: {dash_result.returncode}")
                print(f"dash stdout: {dash_result.stdout.strip()}")
                if dash_result.stderr:
                    print(f"dash stderr: {dash_result.stderr.strip()}")
        elif skip_if_no_dash:
            if self.verbose:
                print("dash not found, skipping comparison")
            return None
        
        # Compare results
        if dash_result:
            success = (
                rs_result.returncode == dash_result.returncode and
                rs_result.stdout == dash_result.stdout and
                rs_result.stderr == dash_result.stderr
            )
            
            if self.verbose:
                if success:
                    print("PASS: Output matches dash")
                else:
                    print("FAIL: Output differs from dash")
            
            return success
        
        return None
    
    def run_test(self, test_name, cmd, expected_exit=0, expected_output=None, 
                 expected_stderr=None, timeout=5):
        """Run a test and check results"""
        if self.verbose:
            print(f"\nTest: {test_name}")
            print(f"Command: {cmd}")
        
        result = self.run_command("rs-dash", cmd, timeout)
        
        if self.verbose:
            print(f"Exit code: {result.returncode} (expected: {expected_exit})")
            print(f"Output: {result.stdout.strip()}")
            if result.stderr:
                print(f"Stderr: {result.stderr.strip()}")
        
        success = True
        
        # Check exit code
        if expected_exit is not None and result.returncode != expected_exit:
            if self.verbose:
                print(f"FAIL: Exit code mismatch")
            success = False
        
        # Check stdout
        if expected_output is not None:
            expected_clean = expected_output.strip() if expected_output else ""
            actual_clean = result.stdout.strip()
            if expected_clean != actual_clean:
                if self.verbose:
                    print(f"FAIL: Output mismatch")
                    print(f"Expected: {expected_clean}")
                    print(f"Got: {actual_clean}")
                success = False
        
        # Check stderr
        if expected_stderr is not None:
            expected_stderr_clean = expected_stderr.strip() if expected_stderr else ""
            actual_stderr_clean = result.stderr.strip()
            if expected_stderr_clean != actual_stderr_clean:
                if self.verbose:
                    print(f"FAIL: Stderr mismatch")
                    print(f"Expected: {expected_stderr_clean}")
                    print(f"Got: {actual_stderr_clean}")
                success = False
        
        if success and self.verbose:
            print("PASS")
        
        return success
    
    def run_interactive_test(self, commands, expected_outputs=None, timeout=5):
        """Run interactive test by feeding commands via stdin"""
        if self.verbose:
            print(f"\nInteractive test with commands: {commands}")
        
        # Convert commands to input string
        input_text = "\n".join(commands) + "\n"
        
        try:
            result = subprocess.run([str(self.rs_dash_path)],
                                  input=input_text,
                                  capture_output=True,
                                  text=True,
                                  encoding='utf-8',
                                  errors='ignore',
                                  timeout=timeout)
            
            if self.verbose:
                print(f"Exit code: {result.returncode}")
                print(f"Output:\n{result.stdout}")
                if result.stderr:
                    print(f"Stderr:\n{result.stderr}")
            
            # Check expected outputs if provided
            if expected_outputs:
                for expected in expected_outputs:
                    if expected not in result.stdout:
                        if self.verbose:
                            print(f"FAIL: Expected output '{expected}' not found")
                        return False
            
            return True
            
        except subprocess.TimeoutExpired:
            if self.verbose:
                print("TIMEOUT: Interactive test hung")
            return False
    
    def create_test_script(self, commands):
        """Create a temporary test script file"""
        with tempfile.NamedTemporaryFile(mode='w', suffix='.sh', delete=False) as f:
            for cmd in commands:
                f.write(f"{cmd}\n")
            script_path = f.name
        
        return script_path
    
    def cleanup_script(self, script_path):
        """Clean up temporary script file"""
        try:
            os.unlink(script_path)
        except OSError:
            pass