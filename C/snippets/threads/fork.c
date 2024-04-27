#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

int main() {
  pid_t pid;

  // Fork a child process
  pid = fork();

  if (pid < 0) {
    fprintf(stderr, "Fork failed\n");
    return 1;
  } else if (pid == 0) {
    execlp("/bin/ls", "ls", NULL);
  }

  else {
    wait(NULL);
    printf("Child completed\n");
  }

  return 0; // Eseguito da padre e figlio (fuori dagli if!)
}