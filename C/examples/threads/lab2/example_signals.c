#include <stdio.h>
#include <unistd.h>
#include <signal.h>

int main(void) {
  int pid1; 
  int pid2;
  pid1 = fork();
  if (pid1 == 0) { /* First child */
    int ch1_pid = getpid();
    while (1) { /* Infinite loop */
      printf("child1 %d is alive\n", ch1_pid);
      sleep(1);
    }
  }
  pid2 = fork();
  if (pid2 == 0) { /* Second child */
    int ch2_pid = getpid();
    while (1) { /* Infinite loop */
      printf("child2 %d is alive\n", ch2_pid);
      sleep(1);
    }
  }
  sleep(3);
  kill(pid1, SIGSTOP); /* Suspend first child */
  sleep(3);
  kill(pid1, SIGCONT); /* Resume first child */
  sleep(3);
  kill(pid1, SIGINT); /* Kill first child */
  kill(pid2, SIGINT); /* Kill second child */
  return 0;
}
