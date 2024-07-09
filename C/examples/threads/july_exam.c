/*
 * This is the base C file for the 2nd mid-season exam, OS ICE UNITN.
 * Please change the name of the file first.
 * Then you see that the file already contains 
 * 	a couple of defines you may find useful (including N, the size of squared matrixes)
 *	the initialization of the two "operand" matrixes
 *	a function to initialize a matrix in shared memory (you may need it),returning a short*
 *	libraries for computing exe_times and a base structure of main that you may also change
 *
 *
 * TASK: you have to implement a matrix multiplication. Sequential, with forked subprocesses, with pthreads and with openmp.
 * in each block you find empty comment lines that you should complete with some explanation of your design choices. 
 * This is very important for the evaluation: a correct code with no or very poor explanation is not gonna be considered correct.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>
#include <omp.h>
#include <sys/time.h>
#include <sys/shm.h>

#define N 1000
#define min(a, b) (((a) < (b)) ? (a) : (b))

// user defined types for pthreads
struct mat_lines {
  int from_row, to_row;
};

typedef struct mat_lines mat_lines_t;

short matrix1[N][N];
short matrix2[N][N];
short result[N][N];

// thread routine for pthreads
void *threader(void *args) {
  mat_lines_t loc = *(mat_lines_t *)args;
  
  for(int i = loc.from_row; i < loc.to_row; i++) {
    for(int j = 0; j < N; j++) {
    short res = 0;	
    for(int k = 0; k < N; k++) {
      res += matrix1[i][k] * matrix2[k][j];
    }
    result[i][j] = res;	
  }
  }   
  pthread_exit(0);
}

// fork subprocess routine
void* fork_threader(int my_pipe[], int from, int to, short* sh_result) {
  // compute the result of one value in results matrix
  for(int i = from; i < to; i++) {
    for(int j = 0; j < N; j++) {
    short res = 0;	
    for(int k = 0; k < N; k++) {
      res += matrix1[i][k] * matrix2[k][j];
    }
    *(sh_result + i*N + j) = res;	
  }
  }   
  int mypid = getpid();
  close(my_pipe[0]);
  write(my_pipe[1], &mypid, sizeof(int));
}

// creates shared memory matrix for forks
short* create_shared_matrix_result(){

  // Creates area of memory of size NxNxsizeof(short)
  int shmc = shmget (87, N*N*sizeof (short), IPC_CREAT| 0666);
  if (shmc < 0)
  {
    perror ("shmget");
    exit(1);
  }

  /* Attach the segment as a short array */
  short *matrixC = shmat (shmc, NULL, 0);
  if (matrixC < (short*) NULL)
  {
    perror ("shmat");
    exit(1);
  }
  
  return matrixC;

}


// Base MAIN structure
int main(int argc, char *argv[]) {

  // -------------------------------- INPUT PROCESSING --------------------------------------
  if (argc != 2) {
  fprintf(stderr,"usage: scriptname <integer num_threads>\n");
  return -1;
  }
  int n_threads = atoi(argv[1]);

  // -------------------------------- INIT MATRIXES --------------------------------------
  for(int i = 0; i < N; i++) {
    for(int j = 0; j < N; j++) {
      matrix1[i][j] = ((i+1) * (j+1) % 100);
      matrix2[i][j] = ((i+1) * (j+1) % 100);
    }
  }
  
  // -------------------------------- SEQUENTIAL CODE --------------------------------------
  // Hint: remember that the squared matrix multiplication works as follows
  // res_i,j = sum[k=1 to N] ( m1_i,k * m2_k,j )
  struct timeval  tv1, tv2;
  gettimeofday(&tv1, NULL);
  
  for(int i = 0; i < N; i++) {
    for(int j = 0; j < N; j++) {
      short res = 0;
      for(int k = 0; k < N; k++) {
      res += matrix1[i][k] * matrix2[k][j];
      }
      result[i][j] = res;
    }
  }  
  
  gettimeofday(&tv2, NULL);
  double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
  printf("Elapsed time with %dx%d result matrix shape, sequential, took %.3f secs\n", N, N, cpu_time_used);
  
  // -------------------------------- FORK CODE --------------------------------------
  // Hint: remember that each process has its own (local) copy of variables
  short* sh_result = create_shared_matrix_result();
  
  // start computations
  gettimeofday(&tv1, NULL);
  int pipes[n_threads][2]; 
  
  // Spawning Childs
    for(int i=0;i<n_threads;i++){
      pipe(pipes[i]);
      int pid = fork();
      if (pid < 0) { /* enters here if error occurs */
      fprintf(stderr, "Fork Failed\n");
      return 1;
      }
      else if (pid == 0) { /* child process, computes a chunk of the result matrix in the shared memory*/
      int from = i*(N/n_threads+1);
      int to = min((i+1)*(N/n_threads+1), N);
      fork_threader(pipes[i], from, to, sh_result);
      return 0;
      }
    }
    
    // Wait for childs to finish. Could have been made using wait or waitpid
    int pipe_out;
    for(int i=0;i<n_threads;i++){
      close(pipes[i][1]);
      read(pipes[i][0], &pipe_out, sizeof(int));
      //printf("Child #%d finished\n", i);
    }
  
  gettimeofday(&tv2, NULL);
  cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
  printf("Elapsed time with %dx%d result matrix shape, with %d subprocesses, took %.3f secs\n", N, N, n_threads, cpu_time_used);
  
  // -------------------------------- PTHREADS CODE --------------------------------------
  // Hint: dont forget the -lpthread flag while compiling
  gettimeofday(&tv1, NULL);
  
  mat_lines_t thread_args[n_threads];
  pthread_t threads[n_threads];
  for(int i = 0; i < n_threads; i++) {
    thread_args[i].from_row = i*(N/n_threads+1);
  thread_args[i].to_row = min((i+1)*(N/n_threads+1), N);
  pthread_create(&threads[i], NULL, threader, &thread_args[i]);
  }
  
  // wait for remaining threads to finish
  for(int i = 0; i < n_threads; i++) {
    //fprintf(stdout, "\twaiting for thread\n");
    pthread_join(threads[i], NULL);
  }
  
  gettimeofday(&tv2, NULL);
  cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
  printf("Elapsed time with %dx%d result matrix shape, with %d pthreads, took %.3f secs\n", N, N, n_threads, cpu_time_used);
  
  // -------------------------------- OMP CODE --------------------------------------
  // Hint: dont forget the -fopenmp flag, and to set the number of threads either in code or in the shell
  gettimeofday(&tv1, NULL);
  
  #pragma omp parallel for private(matrix1, matrix2)
  for(int i = 0; i < N; i++) {
  for(int j = 0; j < N; j++) {
    short res = 0;
    for(int k = 0; k < N; k++) {
    res += matrix1[i][k] * matrix2[k][j];
    }
    result[i][j] = res;
  }
  }
  
  gettimeofday(&tv2, NULL);
  cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
  printf("Elapsed time with %dx%d result matrix shape, with omp threads, took %.3f secs\n", N, N, cpu_time_used);
  
  return 0;
}

