#include <stdio.h>

int fibonacci(int n) {
    if (n <= 1)
        return n;
    else
        return fibonacci(n-1) + fibonacci(n-2);
}

int fibonacci2(int n) {
  int a = 0, b = 1, temp, i;

  if (n == 0) return a;
  if (n == 1) return b;

  for (i = 2; i <= n; i++) {
    temp = a + b;
    a = b;
    b = temp;
  }

  return b;
}

int main() {
    int n, i;

    printf("Insert a number >> ");
    scanf("%d", &n);

    printf("Serie di Fibonacci:\n");
    for (i = 0; i < n; i++) {
        printf("%d ", fibonacci(i));
    }

    return 0;
}