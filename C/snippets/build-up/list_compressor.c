#include <stdio.h>
#include <stdlib.h>

char* list_compressor(int* array, int len) {
  int* out = (int*) malloc(len * sizeof(int));
  int k = 0;
  out[k] = sprintf(&(out[k++]), "%d", array[0]);

  int check;
  int i;
  for (i = 1; i < len; i ++) {
    check = array[i];
    if (array[i] = array[i-1] +1) continue;

  out[k] = sprintf(&(out[k++]), "%d", array[i]);
  }
}

int main() {
  int array[] = {1, 2};
  printf("%d\n", *(&array[0] + 1));
}