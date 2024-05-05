#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>

// To be excuted by 2 childs
void m_worker(int my_pipe[2], int *matrix, int N){
	int sum = 0;
	for(int i = 0; i < N; i++) {
		for(int j = 0; j < N; j++) {
			sum = sum + *(matrix + i*N + j);
		}
	}   
	close(my_pipe[0]);
	write(my_pipe[1], &sum, sizeof(int));
}

// To be excuted by 1 child
void p_worker(int my_pipe[2], int *matrix1, int *matrix2, int N){
	int sum = 0;
	for(int i = 0; i < N; i++) {
		for(int j = 0; j < N; j++) {
			int res = 0;	
			for(int k = 0; k < N; k++) {
				res = res + *(matrix1 + i*N + k) * *(matrix2 + k*N + j);
			}
			sum = sum + res;
		}
	}   
	close(my_pipe[0]);
	write(my_pipe[1], &sum, sizeof(int));
}


int main(int argc, char *argv[]) {

	// input params
	if (argc != 2) {
		fprintf(stderr,"usage: exe_name <n>\n");
	     	/*exit(1);*/
	     	return -1;
	}
    	int N = atoi(argv[1]);
    
	// Used vars
	int i, j, k; 
	int matrix1[N][N];
	int matrix2[N][N];

	// seed values in operand matrixes
	for(i = 0; i < N; i++) {
		for(j = 0; j < N; j++) {
		    	matrix1[i][j] = rand() % 10;
		    	matrix2[i][j] = rand() % 10;
		}
	}

    	// getting start time  
	struct timeval  tv1, tv2;
	gettimeofday(&tv1, NULL);
	
	int pipes[3][2]; 
	
	// Spawning Childs: child0 does sum_m1, child 1 does sum_m2, child 2 does sum_prod
    	for(i=0;i<3;i++){
    		pipe(pipes[i]);
	    	int pid = fork();
	    	if (pid < 0) { /* error occurred */
			fprintf(stderr, "Fork Failed\n");
			return 1;
	    	}
	    	else if (pid == 0) { 
			switch(i){
				case 0:
					// computes m1 sum
					m_worker(pipes[i], matrix1, N);
					return 0;
				case 1:
					// computes m2 sum
					m_worker(pipes[i], matrix2, N);
					return 0;
				case 2:
					// compute prod sum
					p_worker(pipes[i], matrix1, matrix2, N);
					return 0;
				default:
					printf("ERR: Unexpected i value\n");
				
			}
			
        	}
    	}
    	
    	// Wait for childs to finish
    	int pipe_out;
    	for(i=0;i<3;i++){
    		close(pipes[i][1]);
    		read(pipes[i][0], &pipe_out, sizeof(int));
    		printf("Child #%d finished with output %d\n", i, pipe_out);
    	}
 
	gettimeofday(&tv2, NULL);
	double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);

	printf("Elapsed time: took %.3f secs\n", cpu_time_used);
    	return 0;
}

