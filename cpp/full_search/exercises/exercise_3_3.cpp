#include <iostream>
#include <vector>

using namespace std;

int main() {
  int N;
  cin >> N;

  vector<int> a(N);
  for (int i = 0; i < N; ++i) {
	cin >> a[i];
  }

  int fm = a[0];
  int sm = a[1];
  for (int i = 1; i < N; ++i) {
	if (fm > a[i]) {
	  if (sm > a[i]) {
		sm = fm;
	  }
	  fm = a[i];
	}
  }

  cout << sm << endl;
}
