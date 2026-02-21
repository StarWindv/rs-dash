@echo off
REM Simple test script for rs-dash (Windows)

echo === Testing rs-dash ===
echo.

REM Build rs-dash
echo Building rs-dash...
cargo build

if %errorlevel% neq 0 (
    echo Build failed!
    exit /b 1
)

echo Build successful!
echo.

REM Run the main test suite
echo Running test suite...
python test\run_all_tests.py

exit /b %errorlevel%