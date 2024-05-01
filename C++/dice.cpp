#include <iostream> // Drops .h (still available tho)
#include <cstdlib>
#include <ctime>

using namespace std;

const int sides = 6;  // #define SIDES 6

// Direttiva compiler
// I have a function, I don't want a function call!
inline int r_sides() {
  return (rand() % sides +1);
}

int main(void) {
  const int n_dice = 2;
  int d1, d2;

  srand(clock());
  cout << "\nEnter number of trials : ";
  int trials;

  // Type safety implicita!
  cin >> trials;  // Compared to scanf()

  // Per rappresentare i numeri da 0 a 12 (13 valori!)
  int* outcomes = new int[n_dice * sides +1];

  for (int j = 0; j < trials; ++j) {
    outcomes[(d1 = r_sides()) + (d2 = r_sides())]++;
    cout << "Probability\n";

    for (int j = 2; j < n_dice * sides +1; ++j) {
      cout << "j = " << j << "p = "
        << static_cast<double> (outcomes[j] / trials)
        << endl;
    }
  }
}