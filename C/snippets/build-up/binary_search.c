#include <stdio.h>
#include <limits.h>
// #include <time.h>

#define ARRAY_LEN 5

int binary_search(int array[], int l, int r, int x) {
  if (l <= r) {
    int mid = l + (r - l) / 2;
    if (array[mid] == x)
      return mid;

    if (array[mid] > x)
      return binary_search(array, l, mid -1, x);

    return binary_search(array, l, mid +1, x);
  }

  return -1;
}

void custom_sort(int* array, int len) {
  int i, j;
  for (i = 0; i < len - 1; i++) {
    for (j = i; j < len; j++) {
      if (array[i] > array[j]) {
        int temp = array[i];

        array[i] = array[j];
        array[j] = temp;
      }
    }
  }
}

void swap(int *xp, int *yp) {
  int temp = *xp;
  *xp = *yp;
  *yp = temp;
}

void bubble_sort(int arr[], int len) {
  int i, j;
  for (i = 0; i < len -1; i++) {
    // Bigger element goes to the end
    for (j = 0; j < len -i -1; j++) {
      if (arr[j] > arr[j+1]) {
        swap(&arr[j], &arr[j+1]);
      }
    }
  }
}

int main() {
  int array[] = {31, 53, 11, 74, 22};

  bubble_sort(array, ARRAY_LEN);

  int i;
  for (i = 0; i < ARRAY_LEN; i++)
    printf("%d ", array[i]);

  printf("\n");
  return 0;
}