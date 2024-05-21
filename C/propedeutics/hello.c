#include <stdio.h>
#include <stdlib.h>

void print_array(char* pointer) {
  int i;
  for (i = 0; pointer[i] != '\0'; i++) { // Controlliamo il valore dell'array, non l'indice
    printf("%c", pointer[i]);
  }

  printf("\n");
}

int main() {
  char string[10];
  
  int i;
  for (i = 0; i < 9; i++) { // Assicurati di riempire solo fino al penultimo indice
    string[i] = 97 + i;  // Il primo valore sarà 'a'
  }
  string[9] = '\0'; // Aggiungi il null terminatore alla fine della stringa

  print_array(string);
  return 0;
}
