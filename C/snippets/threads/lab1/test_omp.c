#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <omp.h>

#define ARRAYSIZE 2000000000
short array[ARRAYSIZE];

int main(void) {

    unsigned i;
    
    struct timeval  tv1, tv2;
    gettimeofday(&tv1, NULL);

    #pragma omp parallel for private(i), shared (array) 
    for (i=0; i<ARRAYSIZE; i++)
    {
        array[i]=(i*i)%100;
    }
    
    gettimeofday(&tv2, NULL);
    double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
    
    printf("Elapsed time: took %.3f secs\n", cpu_time_used);

    return 0;
}

