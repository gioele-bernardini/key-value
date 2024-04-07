// gcc <file name> -o <program name> -lm
// sudo apt-get install libgmp3-dev -y
#include <stdio.h>
#include <gmp.h>
#include <math.h>

#define BYTE_AREA ((int)(pow(2, 33) / sizeof(int)))
#define SIZE_RATIO 4.5

/*
R = L / H
A = L * H => H = A / L
R = L / (A / L) => R = L * (L / A) => R = L^2 / A
Hence, L = sqrt(R * A)
*/

int get_height() {
  int height = sqrt((double) BYTE_AREA / SIZE_RATIO);

  return height;
}

int main() {
  return 0;
}