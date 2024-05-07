#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <threads.h>

// To be executed by each child
void fork_worker() {
  int my_pid = getpid();
  printf("Hi from child %d\n", my_pid);
  sleep(1);
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

  // Spawning Childs
  for(int i=0; i<n_threads; i++) {
    int pid = fork();
    if (pid < 0) { /* error occurred */
      fprintf(stderr, "Fork Failed\n");
      return 1;
    }
    else if (pid == 0) { 
      fork_worker();
      return 0;
    }
  }

  // Wait for childs to finish
  pid_t wpid;
  while((wpid=wait(NULL)) > 0) {
    printf("Child %d ended\n", wpid);
  }

  gettimeofday(&tv2, NULL);
  double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);

  printf("Elapsed time: took %.3f secs\n", cpu_time_used);
  return 0;
}
