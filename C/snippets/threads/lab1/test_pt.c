#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <sys/time.h>

#define ARRAYSIZE 2000000000
short array[ARRAYSIZE];

struct ThreadData {
  unsigned start;
  unsigned stop;
  short* array;
};

/* puts i^2 into array positions i=start to stop-1 */
void* squarer(void* td) {
  struct ThreadData* data = (struct ThreadData*)td;
  unsigned start = data->start;
  unsigned stop = data->stop;
  short* array = data->array;
  unsigned i;
  for (i = start; i < stop; i++) {
    array[i] = (i * i) % 100;
  }

  return NULL;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    fprintf(stderr, "usage: a.out <integer value>\n");
    /*exit(1);*/
    return -1;
  }

  int n_threads = atoi(argv[1]);

  pthread_t thread[n_threads];
  struct ThreadData data[n_threads];
  int i;

  struct timeval tv1, tv2;
  gettimeofday(&tv1, NULL);

  int tasksPerThread = (ARRAYSIZE + n_threads - 1) / n_threads;

  /* Divide work for threads, prepare parameters */
  /* This means in my example I divide the loop into 8 regions: 0 ..1250,1250 .. 2500 etc., 2500 .. 3750 */

  for (i = 0; i < n_threads; i++) {
    data[i].start = i * tasksPerThread;
    data[i].stop = (i + 1) * tasksPerThread;
    data[i].array = array;

    data[n_threads - 1].stop = ARRAYSIZE;
  }

  for (i = 0; i < n_threads; i++) {
    pthread_create(&thread[i], NULL, squarer, &data[i]);
  }

  for (i = 0; i < n_threads; i++) {
    pthread_join(thread[i], NULL);
  }

  gettimeofday(&tv2, NULL);
  double cpu_time_used = (double)(tv2.tv_usec - tv1.tv_usec) / 1000000 + (double)(tv2.tv_sec - tv1.tv_sec);

  printf("Elapsed time: took %.3f secs with %d threads\n", cpu_time_used, n_threads);
}
