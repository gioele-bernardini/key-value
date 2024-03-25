#include <stdio.h>
#include <stdlib.h>

int factorial(int n) {
  if (n == 1 || n == 0) {
    return 1;
  }

  return n * factorial(n - 1);
}

int main(int argc, char** argv) {
  if (argc != 2);

  printf("%d\n", factorial(atoi(argv[1])));
  return 0;
}
