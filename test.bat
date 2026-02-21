@echo off
REM Test script for rs-dash on Windows

echo Testing rs-dash basic functionality...

REM Test echo command
echo Test 1: echo command
target\debug\rs-dash.exe -c "echo hello world"

REM Test pwd command
echo Test 2: pwd command
target\debug\rs-dash.exe -c "pwd"

REM Test cd and pwd
echo Test 3: cd command
target\debug\rs-dash.exe -c "cd .. && pwd"

REM Test external command
echo Test 4: external command
target\debug\rs-dash.exe -c "dir" | head -5

REM Test help command
echo Test 5: help command
target\debug\rs-dash.exe -c "help"

echo Testing complete!