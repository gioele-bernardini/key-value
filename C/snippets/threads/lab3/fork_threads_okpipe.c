#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <threads.h>

# define MSG_SIZE 2

// To be excuted by each child
void fork_worker(int my_pipe[]){
	int my_pid = getpid();
	printf("Hi from child %d\n", my_pid);
	sleep(1);
	close(my_pipe[0]);
	write(my_pipe[1], "hi", MSG_SIZE);
}


int main(int argc, char *argv[]) {

	if (argc != 2) {
		fprintf(stderr,"usage: exe_name <n_threads>\n");
	     	/*exit(1);*/
	     	return -1;
	}

  	int n_threads = atoi(argv[1]);    
	struct timeval  tv1, tv2;
	gettimeofday(&tv1, NULL);
	
	int pipes[n_threads][2]; 
	
	// Spawning Childs
    	for(int i=0;i<n_threads;i++){
    		pipe(pipes[i]);
	    	int pid = fork();
	    	if (pid < 0) { /* error occurred */
			fprintf(stderr, "Fork Failed\n");
			return 1;
	    	}
	    	else if (pid == 0) { /* child process, computes the first half */
			fork_worker(pipes[i]);
			return 0;
        	}
    	}
    	
    	// Wait for childs to finish
    	for(int i=0;i<n_threads;i++){
    		close(pipes[i][1]);
    		char pipe_out[MSG_SIZE];
    		read(pipes[i][0], pipe_out, MSG_SIZE);
    		printf("Child #%d finished with output %s\n", i, pipe_out);
    	}
 
	gettimeofday(&tv2, NULL);
	double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);

	printf("Elapsed time: took %.3f secs\n", cpu_time_used);
    	return 0;
}

