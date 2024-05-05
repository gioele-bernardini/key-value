#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <unistd.h>

# define FILENAME "my_file_nothreads.txt"

int count;

int main(int argc, char *argv[]){


  if (argc != 2) {
     fprintf(stderr,"usage: a.out <max_value>\n");
     /*exit(1);*/
     return -1;
  }
  remove(FILENAME);
  
  int max_value = atoi(argv[1]);
  
  struct timeval  tv1, tv2;
  gettimeofday(&tv1, NULL);
  
  count = 0;
  for (int i=0;i<max_value;i++){
    	count = count + 1;
    	FILE *fptr = fopen(FILENAME, "a");
    	fprintf(fptr, "single-thread has updated count to %d\n", count);
    	fclose(fptr);
  }
  

  gettimeofday(&tv2, NULL);
  double cpu_time_used = (double) (tv2.tv_usec - tv1.tv_usec) / 1000000 + (double) (tv2.tv_sec - tv1.tv_sec);
    
  printf("Elapsed time: took %.3f secs\n", cpu_time_used);
  printf("\n-------------------------------------------------------------------------\n");
}
