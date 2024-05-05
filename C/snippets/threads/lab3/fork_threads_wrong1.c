#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <threads.h>

int active_threads;

// To be excuted by each child
void fork_worker(){
	int my_pid = getpid();
	printf("Hi from child %d\n", my_pid);
	sleep(1);
}

// This does not work because each child has its own copy of "active threads"
int main(int argc, char *argv[]) {

	if (argc != 2) {
		fprintf(stderr,"usage: exe_name <n_threads>\n");
	     	/*exit(1);*/
	     	return -1;
	}

  	int n_threads = atoi(argv[1]);    
	struct timeval  tv1, tv2;
	gettimeofday(&tv1, NULL);
	
	// Spawning Childs
    	for(int i=0;i<n_threads;i++){
	    	int pid = fork();
	    	if (pid < 0) { /* error occurred */
			fprintf(stderr, "Fork Failed\n");
			return 1;
	    	}
	    	else if (pid == 0) { 
			fork_worker();
			active_threads--;
			return 0;
        	}
        	active_threads++;
    	}
    	
    	// Wait for childs to finish
    	while(active_threads > 0){
    		printf("Waiting for threads, still %d alive\n", active_threads);
    		sleep(1);
    	}
 
	gettimeofday(&tv2, NULL);
	double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);

	printf("Elapsed time: took %.3f secs\n", cpu_time_used);
    	return 0;
}

