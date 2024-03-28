#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define ARRAY_SIZE 17
#define MAX_VALUE 100
#define MIN_VALUE 1

void shift_array_from(int* array, int index, int len) {

  int i;
  for (i = index; i < len -1; i++)
    array[i] = array[i+1];

  // for (i=0; i<len-1; i++)
    // printf("%d ", array[i]);
  // printf("\n");
}

int remove_duplicates(int* array, int len) {
  int i, j;
  int new_len = len;

  for (i = 0; i < new_len - 1; i++) {
    for (j = i + 1; j < new_len; j++) {
      if (array[i] == array[j]) {
        printf("Found duplicate => %d\n", array[i]);
        shift_array_from(array, j, new_len);
        new_len--;
      }
    }
  }

  if (new_len == len)
    printf("No duplicates found.\n");

  return new_len;
}

int main() {
  srand(time(NULL));

  int array[ARRAY_SIZE];

  int i;
  for (i = 0; i < ARRAY_SIZE; i++)
    array[i] = (rand() % (MAX_VALUE - MIN_VALUE + 1)) + MIN_VALUE;

  int new_len = remove_duplicates(array, ARRAY_SIZE);

  printf("New length => %d\n", new_len);
  printf("[ ");
  for (i = 0; i < new_len; i++)
    printf("%d ", array[i]);
  printf(" ]\n");
}
