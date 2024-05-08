#include <stdio.h>

void reverse(int* array, int len) {
  int temp = 0;

  int i = 0;
  // fino a meta'! lunghezza devo saperla a priori 
  // se e' 6 ottengo 3 (i < 3 nel ciclo for)
  // se invece e' dispari (5) => non cambia niente :)
  // (il valore in mezzo verra' ignorato!)

  for (i; i<(len/2); i++) {
    temp = array[len-1 - i];
    array[len-1 -i] = array[i];

    array[i] = temp;
  }

  for (i=0; i<len; i++) {
    printf("%d ", array[i]);
  }
}

int main() {
  int array[] = {14, 62, 64, 32, 9, 32};

  reverse(array, 6);

  return 0;
}