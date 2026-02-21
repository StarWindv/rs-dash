#!/usr/bin/env python3
"""
POSIX Shell Conformance Tests for rs-dash

These tests verify that rs-dash conforms to POSIX shell standards
and behaves like native dash where applicable.
"""

import sys
import os
import subprocess
import tempfile
sys.path.append(os.path.dirname(os.path.dirname(__file__)))

from test_utils import TestRunner

class POSIXConformanceTester:
    """Test POSIX shell conformance"""
    
    def __init__(self):
        self.runner = TestRunner(verbose=True)
        self.test_results = []
    
    def run_test(self, description, command, expected_output=None, expected_exit=0, 
                 skip_if_no_dash=True, posix_required=True):
        """Run a conformance test"""
        print(f"\n{'='*60}")
        print(f"Test: {description}")
        print(f"Command: {command}")
        print(f"POSIX Required: {posix_required}")
        
        # Run with rs-dash
        rs_result = self.runner.run_command("rs-dash", command)
        
        print(f"rs-dash exit: {rs_result.returncode}")
        print(f"rs-dash output: {rs_result.stdout.strip()}")
        if rs_result.stderr:
            print(f"rs-dash stderr: {rs_result.stderr.strip()}")
        
        # Try to run with native dash for comparison
        dash_result = None
        if self.runner.dash_available and not skip_if_no_dash:
            dash_result = self.runner.run_command("dash", command)
            print(f"dash exit: {dash_result.returncode}")
            print(f"dash output: {dash_result.stdout.strip()}")
            if dash_result.stderr:
                print(f"dash stderr: {dash_result.stderr.strip()}")
        
        # Evaluate results
        test_passed = True
        issues = []
        
        # Check exit code
        if expected_exit is not None and rs_result.returncode != expected_exit:
            issues.append(f"Exit code mismatch: expected {expected_exit}, got {rs_result.returncode}")
            test_passed = False
        
        # Check output
        if expected_output is not None:
            expected_clean = expected_output.strip() if expected_output else ""
            actual_clean = rs_result.stdout.strip()
            if expected_clean != actual_clean:
                issues.append(f"Output mismatch: expected '{expected_clean}', got '{actual_clean}'")
                test_passed = False
        
        # Compare with dash if available
        if dash_result and not skip_if_no_dash:
            if rs_result.returncode != dash_result.returncode:
                issues.append(f"Exit code differs from dash: rs-dash={rs_result.returncode}, dash={dash_result.returncode}")
                test_passed = False
            
            if rs_result.stdout != dash_result.stdout:
                issues.append(f"Output differs from dash")
                # Don't fail if output differs but rs-dash behavior is acceptable
                # Just note the difference
        
        # Record result
        result = {
            'description': description,
            'command': command,
            'passed': test_passed,
            'issues': issues,
            'posix_required': posix_required,
            'dash_available': self.runner.dash_available,
            'dash_match': dash_result is not None and 
                         rs_result.returncode == dash_result.returncode and
                         rs_result.stdout == dash_result.stdout
        }
        
        self.test_results.append(result)
        
        if test_passed:
            print("PASS")
        else:
            print("FAIL")
            for issue in issues:
                print(f"  - {issue}")
        
        return test_passed
    
    def run_basic_syntax_tests(self):
        """Test basic shell syntax"""
        print("\n" + "="*60)
        print("BASIC SYNTAX TESTS")
        print("="*60)
        
        tests = [
            # Empty command
            ("Empty command", "", "", 0, True, True),
            
            # Simple echo
            ("Simple echo", "echo hello", "hello", 0, True, True),
            
            # Echo with multiple arguments
            ("Multiple arguments", "echo hello world", "hello world", 0, True, True),
            
            # Empty echo
            ("Empty echo", "echo", "", 0, True, True),
            
            # Multiple spaces
            ("Multiple spaces", "echo   hello   world", "hello world", 0, True, True),
            
            # Tabs as whitespace
            ("Tabs in command", "\techo\thello\tworld", "hello world", 0, False, True),
            
            # Trailing whitespace
            ("Trailing whitespace", "echo hello ", "hello", 0, False, True),
            
            # Leading whitespace
            ("Leading whitespace", " echo hello", "hello", 0, False, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_variable_tests(self):
        """Test variable assignment and expansion"""
        print("\n" + "="*60)
        print("VARIABLE TESTS")
        print("="*60)
        
        tests = [
            # Simple variable assignment
            ("Simple assignment", "VAR=value", "", 0, True, True),
            
            # Variable expansion
            ("Variable expansion", "VAR=test; echo $VAR", "test", 0, True, True),
            
            # Variable with && operator
            ("Variable with &&", "VAR=test && echo $VAR", "test", 0, True, True),
            
            # Multiple variables
            ("Multiple variables", "A=1 B=2; echo $A $B", "1 2", 0, True, True),
            
            # Variable with no value
            ("Empty variable", "EMPTY=; echo $EMPTY", "", 0, True, True),
            
            # Undefined variable (should expand to empty string)
            ("Undefined variable", "echo $UNDEFINED", "", 0, True, True),
            
            # Variable name with underscore
            ("Underscore variable", "MY_VAR=test; echo $MY_VAR", "test", 0, True, True),
            
            # Variable with digits
            ("Variable with digits", "VAR1=test; echo $VAR1", "test", 0, True, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_special_variable_tests(self):
        """Test special shell variables"""
        print("\n" + "="*60)
        print("SPECIAL VARIABLE TESTS")
        print("="*60)
        
        tests = [
            # Exit status of true
            ("Exit status true", "true; echo $?", "0", 0, True, True),
            
            # Exit status of false
            ("Exit status false", "false; echo $?", "1", 0, True, True),
            
            # Shell PID (should be a number)
            ("Shell PID", "echo $$", None, 0, True, True),
            
            # Shell name
            ("Shell name", "echo $0", "rs-dash", 0, False, True),
            
            # Positional parameters (when provided)
            ("Positional params", 'echo "$1 $2"', "arg1 arg2", 0, False, True),
            
            # Number of positional parameters
            ("Param count", 'echo $#', "2", 0, False, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_command_substitution_tests(self):
        """Test command substitution"""
        print("\n" + "="*60)
        print("COMMAND SUBSTITUTION TESTS")
        print("="*60)
        
        tests = [
            # Basic command substitution
            ("Basic substitution", "echo $(echo test)", "test", 0, True, True),
            
            # Command substitution with arguments
            ("Substitution with args", "echo $(echo hello world)", "hello world", 0, True, True),
            
            # Substitution in middle of string
            ("Substitution in middle", "echo prefix$(echo middle)suffix", "prefixmiddlesuffix", 0, True, True),
            
            # Multiple substitutions
            ("Multiple substitutions", "echo $(echo first) $(echo second)", "first second", 0, True, True),
            
            # Nested substitution
            ("Nested substitution", "echo $(echo $(echo nested))", "nested", 0, True, True),
            
            # Substitution with variables
            ("Substitution with var", "MSG=hello; echo $(echo $MSG world)", "hello world", 0, True, True),
            
            # Empty command substitution
            ("Empty substitution", "echo $(echo)", "", 0, True, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_quote_tests(self):
        """Test quoting rules"""
        print("\n" + "="*60)
        print("QUOTING TESTS")
        print("="*60)
        
        tests = [
            # Single quotes (no expansion)
            ("Single quotes", "echo '$HOME'", "$HOME", 0, True, True),
            
            # Double quotes (expansion happens)
            ("Double quotes", 'echo "$HOME"', None, 0, True, True),  # $HOME should expand
            
            # Mixed quotes
            ("Mixed quotes", "echo 'single' \"double\"", "single double", 0, True, True),
            
            # Escaped dollar sign
            ("Escaped $", "echo \\$HOME", "$HOME", 0, True, True),
            
            # Quotes in command substitution
            ("Quotes in substitution", 'echo "$(echo test)"', "test", 0, True, True),
            
            # Nested quotes
            ("Nested quotes", 'echo "outer \'inner\' outer"', "outer 'inner' outer", 0, True, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_operator_tests(self):
        """Test command operators"""
        print("\n" + "="*60)
        print("OPERATOR TESTS")
        print("="*60)
        
        tests = [
            # Semicolon (sequential execution)
            ("Semicolon", "echo first; echo second", "first\nsecond", 0, True, True),
            
            # AND operator (&&)
            ("AND success", "true && echo success", "success", 0, True, True),
            ("AND failure", "false && echo no", "", 1, True, True),
            
            # OR operator (||)
            ("OR failure", "false || echo yes", "yes", 0, True, True),
            ("OR success", "true || echo no", "", 0, True, True),
            
            # Combined operators
            ("Combined operators", "false || true && echo mixed", "mixed", 0, True, True),
            
            # Exit status with operators
            ("Exit status chain", "false && true; echo $?", "1", 0, True, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        return passed, total - passed
    
    def run_redirection_tests(self):
        """Test I/O redirection"""
        print("\n" + "="*60)
        print("REDIRECTION TESTS")
        print("="*60)
        
        # Create temporary files
        with tempfile.NamedTemporaryFile(mode='w', delete=False) as tmp_out:
            tmp_out_path = tmp_out.name
        
        with tempfile.NamedTemporaryFile(mode='w', delete=False) as tmp_in:
            tmp_in.write("input data\n")
            tmp_in_path = tmp_in.name
        
        tests = [
            # Output redirection
            (f"Output redirect", f"echo test > {tmp_out_path}", "", 0, False, True),
            
            # Append redirection
            (f"Append redirect", f"echo more >> {tmp_out_path}", "", 0, False, True),
            
            # Input redirection
            (f"Input redirect", f"cat < {tmp_in_path}", "input data", 0, False, True),
        ]
        
        passed = 0
        total = len(tests)
        
        for desc, cmd, expected, exit_code, skip_dash, posix_req in tests:
            if self.run_test(desc, cmd, expected, exit_code, skip_dash, posix_req):
                passed += 1
        
        # Clean up
        import os
        try:
            os.unlink(tmp_out_path)
            os.unlink(tmp_in_path)
        except:
            pass
        
        return passed, total - passed
    
    def run_summary(self):
        """Print test summary"""
        print("\n" + "="*60)
        print("CONFORMANCE TEST SUMMARY")
        print("="*60)
        
        total_tests = len(self.test_results)
        passed_tests = sum(1 for r in self.test_results if r['passed'])
        failed_tests = total_tests - passed_tests
        
        posix_tests = sum(1 for r in self.test_results if r['posix_required'])
        posix_passed = sum(1 for r in self.test_results if r['posix_required'] and r['passed'])
        
        dash_comparisons = sum(1 for r in self.test_results if r['dash_available'] and not r.get('skip_if_no_dash', True))
        dash_matches = sum(1 for r in self.test_results if r.get('dash_match', False))
        
        print(f"Total tests: {total_tests}")
        print(f"Passed: {passed_tests}")
        print(f"Failed: {failed_tests}")
        print(f"Success rate: {passed_tests/total_tests*100:.1f}%")
        print()
        print(f"POSIX required tests: {posix_tests}")
        print(f"POSIX passed: {posix_passed}")
        print(f"POSIX compliance: {posix_passed/posix_tests*100:.1f}%")
        print()
        if dash_comparisons > 0:
            print(f"Dash comparisons: {dash_comparisons}")
            print(f"Dash matches: {dash_matches}")
            print(f"Dash compatibility: {dash_matches/dash_comparisons*100:.1f}%")
        
        # Print failed tests
        if failed_tests > 0:
            print("\nFAILED TESTS:")
            for result in self.test_results:
                if not result['passed']:
                    print(f"\n- {result['description']}")
                    print(f"  Command: {result['command']}")
                    for issue in result['issues']:
                        print(f"  Issue: {issue}")
        
        return failed_tests == 0

def main():
    """Run all conformance tests"""
    print("POSIX Shell Conformance Tests for rs-dash")
    print("="*60)
    
    tester = POSIXConformanceTester()
    
    # Run test suites
    test_suites = [
        ("Basic Syntax", tester.run_basic_syntax_tests),
        ("Variables", tester.run_variable_tests),
        ("Special Variables", tester.run_special_variable_tests),
        ("Command Substitution", tester.run_command_substitution_tests),
        ("Quoting", tester.run_quote_tests),
        ("Operators", tester.run_operator_tests),
        ("Redirection", tester.run_redirection_tests),
    ]
    
    total_passed = 0
    total_failed = 0
    
    for name, test_func in test_suites:
        passed, failed = test_func()
        total_passed += passed
        total_failed += failed
    
    # Generate summary
    all_passed = tester.run_summary()
    
    if all_passed:
        print("\nAll conformance tests passed!")
        return 0
    else:
        print("\nSome conformance tests failed")
        return 1

if __name__ == "__main__":
    sys.exit(main())