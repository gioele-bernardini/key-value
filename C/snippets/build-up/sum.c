#include <stdio.h>

int main() {
  int A, B, sum = 0;

  while (1) {
    printf("Please insert two valid numbers => ");

    if (scanf("%d %d", &A, &B) != 2) {
      char c;
      while ((c = getchar()) != '\n' && c != EOF);
    } else {
      break;
    }
  }

  sum = A + B;
  printf("The result is => %d\n", sum);

  return 0;
}
