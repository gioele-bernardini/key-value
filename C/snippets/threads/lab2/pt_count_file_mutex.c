#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <sys/time.h>
#include <unistd.h>

#define FILENAME "my_file_withlock.txt"

int count;
pthread_mutex_t count_lock, file_lock;

void* plus_one(void* td) {
  int max = *(int*)td;
  pid_t my_id = gettid();
  int iterations = 0;
  while (count < max) {
    pthread_mutex_lock(&count_lock);
    count = count + 1;
    pthread_mutex_lock(&file_lock);
    FILE *fptr = fopen(FILENAME, "a");
    fprintf(fptr, "thread %d has updated count to %d\n", my_id, count);
    fclose(fptr);
    pthread_mutex_unlock(&file_lock);
    pthread_mutex_unlock(&count_lock);
    iterations = iterations + 1;
  }
  printf("Thread %d existing after doing %d increments\n", my_id, iterations);
  return NULL;
}

int main(int argc, char *argv[]) {

  if (argc != 3) {
    fprintf(stderr, "usage: a.out <n_threads> <max_value>\n");
    /*exit(1);*/
    return -1;
  }
  remove(FILENAME);

  int n_threads = atoi(argv[1]);
  int max_value = atoi(argv[2]);
  pthread_t thread[n_threads];

  struct timeval tv1, tv2;
  gettimeofday(&tv1, NULL);

  count = 0;
  pthread_mutex_init(&count_lock, NULL);
  pthread_mutex_init(&file_lock, NULL);

  for (int i = 0; i < n_threads; i++) {
    pthread_create(&thread[i], NULL, plus_one, &max_value);
  }

  for (int i = 0; i < n_threads; i++) {
    pthread_join(thread[i], NULL);
  }

  gettimeofday(&tv2, NULL);
  double cpu_time_used = (double)(tv2.tv_usec - tv1.tv_usec) / 1000000 + (double)(tv2.tv_sec - tv1.tv_sec);

  printf("Elapsed time: took %.3f secs\n", cpu_time_used);
  printf("\n-------------------------------------------------------------------------\n");

  pthread_mutex_destroy(&count_lock);
  pthread_mutex_destroy(&file_lock);

  return 0;
}
