#include <stdio.h>
#include <stdlib.h>

// N_ROWS = 3
#define N_ROWS 3

// strs = ["flower","flow","flight"]

char* find_prefix(char** strs) { // Come input matrice di char
  int prefix_len = 0;

  int go_on = 1;
  int i, j;

  for (j = 0; strs[0][j] != '\0'; j++) {
    char check = strs[0][j];
    for (i = 1; i < N_ROWS; i++) {
      if (strs[i][j] != check)
        go_on = 0;
    }

    if (!go_on) {
      break;
    }

    printf("%c", strs[0][j]);
    prefix_len++;
    // prefix.append(strs[0][j])
  }

  char* prefix = (char*) malloc((prefix_len +1) * sizeof(char));
  for (j = 0; j < prefix_len; j++) {
    prefix[j] = strs[0][j];
  }
  prefix[j] = '\0';

  return prefix;
}

int main() {
  char* strs[] = {"flower","flow","flight"};

  char* prefix = find_prefix(strs);
  free(prefix);

  return 0;
}

