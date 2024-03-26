#include <stdio.h>
#include <stdlib.h>

void to_binary(int n) {
  int original = n;

  int n_digits = 0;
  while (n) {
    n_digits++;
    n /= 2;
  }

  // printf("Necessary digits => %d\n", n_digits);

  int* ptr = (int*) malloc (n_digits * sizeof(unsigned));

  int i = 0;
  for (i; i<n_digits; i++) {
    ptr[i] = original % 2;
    original /= 2;

    // printf("%u ", ptr[i]);
  }
  // Back-ward cycle for printing
  i = n_digits - 1;
  for (i; i>=0; i--) {
    printf("%d", ptr[i]);
  }

  printf("\n");
}

int main(int argc, char** argv) {
  if (argc != 2);

  to_binary(atoi(argv[1]));

  return 0;
}
