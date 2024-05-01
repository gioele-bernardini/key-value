#include <iostream>
#include <vector>

using namespace std;

const int N = 40;

inline void sum(int& p, int n, const vector<int>& d) {
  p = 0;

  for (int i = 0; i < n; ++i)
    p += d[i];
}

int main(void) {
  int accum = 0;
  vector<int> data(N);

  for (int i = 0; i < N; ++i)
    data[i] = i;

  sum(accum, N, data);
  cout << "Sum is >>> " << accum << endl;

  return 0;
}