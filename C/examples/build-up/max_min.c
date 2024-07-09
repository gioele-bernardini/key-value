#include <stdio.h>

typedef struct tPair {
  int min;
  int max;
} tPair;

tPair max_min(int* array, int len) {
  tPair out;
  out.min = out.max = array[0];

  int i = 0;
  for (i; i<len; i++) {
    if (array[i] < out.min)
      out.min = array[i];
    if (array[i] > out.max)
      out.max = array[i];
  }

  return out;
}

int main() {
  int array[] = {14, 62, 64, 32, 9, 32};

  tPair out = max_min(array, 6);
  printf("%d - %d", out.min, out.max);

  return 0;
}