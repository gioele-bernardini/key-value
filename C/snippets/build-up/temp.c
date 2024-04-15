#include <stdio.h>

void push_array(int* array, int len, int* position) {
  array[*position] = 5;

  *position = (*position +1) % len;
  printf("next position > %d\n", *position);
}

int main() {
  int array[] = {0, 0, 0, 0, 0};
  int len = 5;
  int position = 0;

  push_array(array, len, &position);
  push_array(array, len, &position);
  push_array(array, len, &position);
  push_array(array, len, &position);
  push_array(array, len, &position);
}