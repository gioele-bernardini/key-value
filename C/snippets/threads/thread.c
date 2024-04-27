#include <stdio.h>
#include <pthread.h>
#include <stdlib.h>

int sum;  // This data is shared among threads
void* runner(void *param) {
  int i, upper = atoi(param);
  sum = 0;

  for (i = 1; i <= upper; i++)
    sum += 1;

  pthread_exit(0);
}

int main(int argc, char** argv) {
  pthread_t tid;  // The thread identifier
  pthread_attr_t attr;  // Set of thread attributes

  // Set the default attributes of the thread
  pthread_attr_init(&attr);

  // Create the thread
  pthread_create(&tid, &attr, runner, argv[1]);

  // Wait for the thread to exit
  pthread_join(tid, NULL);

  printf("Sum = %d\n", sum);
}