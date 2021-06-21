#include <iostream>
#include <vector>

using namespace std;

int main() {
  int N, v;
  cin >> N >> v;
  vector<int> as(N);
  for (int i = 0; i < N; ++i) cin >> as[i];

  int count = 0;
  for (int a: as) {
	if (a==v) {
	  ++count;
	}
  }

  cout << count << endl;
}
