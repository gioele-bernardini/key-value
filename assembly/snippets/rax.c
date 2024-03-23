#include <stdio.h>

int main() {
    unsigned long long int rax_value;

    asm ("mov %%rax, %0" : "=r" (rax_value));
    printf("RAX => : %llu\n", rax_value);

    return 0;
}
