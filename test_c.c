#include <stdio.h>
#include <stdlib.h>

int main() {
    // Test 1: Basic parameter expansion
    printf("Test 1: Basic parameter expansion\n");
    system("target\\debug\\rs-dash.exe -c \"VAR=test; echo ${VAR}\"");
    
    // Test 2: Pattern substitution
    printf("\nTest 2: Pattern substitution\n");
    system("target\\debug\\rs-dash.exe -c \"VAR=hello world; echo ${VAR/world/everyone}\"");
    
    // Test 3: Arithmetic expansion
    printf("\nTest 3: Arithmetic expansion\n");
    system("target\\debug\\rs-dash.exe -c \"echo $((1 + 2 * 3))\"");

    return 0;
}