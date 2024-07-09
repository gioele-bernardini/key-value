#include <stdio.h>

int main() {
  int array[10];

  for (int i = 0; i < 10; i++) {
    array[i] = i +1;
  }

  printf("%p\n", array);
  printf("%d\n", *array);
 
  // Stampa il secondo elemento
  printf("%d\n", *(&(array[0]) +1));

  // Stampo il nono elemento
  printf("%d\n", 8[array]);

  int* pointer;
  &pointer = 0x8;

  return 0;
}