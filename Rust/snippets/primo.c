#include <stdio.h>
#include <math.h>

int main() {
    int i, j;
    int is_prime;

    for (i = 2; i < 200; i++) {
        is_prime = 1;

        for (j = 2; j <= sqrt(i); j++) {
            if (i % j == 0) {
                is_prime = 0; 
                break;
            }
        }

        if (is_prime) {
            printf("Numero primo trovato => %d\n", i);
        }
    }

    return 0;
}
