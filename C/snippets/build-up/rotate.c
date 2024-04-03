#include <stdio.h>
#include <stdlib.h>

#define ARRAY_LEN 5

void rotate(int* array, int len) {
  int temp = array[len-1];

  int i = len -1;  // Last element
  for (i; i > 0; i--) {
    array[i] = array[i-1];
  }

  array[0] = temp;

  for (i=0; i<len; i++)
    printf("%d ", array[i]);

  printf("\n");
}

int main(int argc, char** argv) {
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <value>\n", argv[0]);
    exit(1);
  }

  int array[] = {1, 2, 3, 4, 5};
  int i;
  for (i=0; i<ARRAY_LEN; i++)
    printf("%d ", array[i]);

  printf("\n\n==========\n\n");

  int input = atoi(argv[1]), out;
  if (input < 0) {
    out = (ARRAY_LEN - (abs(input) % ARRAY_LEN));
  } else {
    out = input % ARRAY_LEN;
  }

  printf("OUT CALCOLATO => %d\n\n", out);

  for (i=0; i<out; i++)
    rotate(array, ARRAY_LEN);

  return 0;
}
