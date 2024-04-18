#include <stdio.h>

int main() {
  char s[] = "hello";

  int sum = 0, temp;
  int i = 0;
  while (s[i + 1] != '\0') {
    temp = s[i] - s[i + 1];

    if (temp > 0) sum += temp;
    else sum -= temp;

    printf("%d ", sum);
    i++;
  }

  printf("\n total sum >>> %d\n", sum);

  return 0;
}