// gcc <file name> -o <program name> -lm
#include <stdio.h>
#include <math.h>

#define BYTE_AREA ((int)(pow(2, 33) / sizeof(int)))
#define SIZE_RATIO 4.5

/*
R = L / H
A = L * H => H = A / L
R = L / (A / L) => R = L * (L / A) => R = L^2 / A
Hence, L = sqrt(R * A)
*/

float get_height() {
  float height = (float) sqrt((double) SIZE_RATIO * BYTE_AREA);
  printf("Calculated height: %f\n", height);

  return height;
}

int main() {
  float height = get_height();
  printf("Height: %f\n", height);
  
  float length = SIZE_RATIO * height;
  printf("Calculated length: %f\n", length);

  printf("Height * Length: %f\n", height * length);
  printf("Byte Area: %d\n", BYTE_AREA);

  return 0;
}
