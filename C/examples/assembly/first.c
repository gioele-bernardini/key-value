#include <stdio.h>

int main() {
  int a = 5;
  int b = 10;
  int result;

  __asm__ {
    "imull %1, %2;"
    "movl %2, %0;"
    : "=r" (result) // Output
    : "r" (a), "r" (b)  // Input
  }

  printf("Result >>> %d\n", result);

  return 0;
}