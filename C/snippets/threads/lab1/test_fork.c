#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

#define ARRAYSIZE 2000000000
short array[ARRAYSIZE];

int main(void) {
	unsigned i;
	
	struct timeval tv1, tv2;
	gettimeofday(&tv1, NULL);
	
	/* fork a child process */
	int pid = fork();

	if (pid < 0) { /* error occurred */
		fprintf(stderr, "Fork Failed\n");
		return 1;
	} else if (pid == 0) { /* child process, computes the first half */
		for (i = 0; i < ARRAYSIZE / 2; i++) {
			array[i] = (i * i) % 100;
		}
	} else { /* parent process, computes the second half */
		for (i = ARRAYSIZE / 2; i < ARRAYSIZE; i++) {
			array[i] = (i * i) % 100;
		}
		wait(NULL);
		gettimeofday(&tv2, NULL);
		double cpu_time_used = (double)(tv2.tv_usec - tv1.tv_usec) / 1000000 + (double)(tv2.tv_sec - tv1.tv_sec);
	
		printf("Elapsed time: took %.3f secs\n", cpu_time_used);
	}

	return 0;
}
