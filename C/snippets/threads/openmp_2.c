// gcc <file> -o <output> -fopenmp -lgomp

#include <stdio.h>
#include <omp.h>

int main() {
  omp_set_dynamic(0);

  // Default PER TUTTE LE AREE IN PARALLELO..
  omp_set_num_threads(4);

  // Else, we can specify vvvvv a single area
  #pragma omp parallel num_threads(4)
  {
    // 4 threads will be used here..
  }
}

/* Altro metodo
export OMP_NUM_THREADS=4 (ogni nuova shell tho)
*/