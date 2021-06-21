#include <iostream>
#include <vector>
using namespace std;
const int INF = 20000000;

/**
 * # ペアの全探索
 * ## 問題
 * - 与えられたデータの中から最適なペアを探索する問題
 * - 与えられた2組のデータの中からそれぞれ要素を抜き出す方法を最適化する問題
 *
 * ## 計算量
 * - O(N^2)
 *
 * ## 備考
 * - ペアの全探索問題は、二分探索法を用いることで計算量 O(NlogN) で解ける
 */
int main() {
  int N, K;
  cin >> N>>>K;
  vector<int> a(N), b(N);
  for (int i = 0; i < N; ++i) cin >> a[i];
  for (int i = 0; i < N; ++i) cin >> b[i];

  int min_value = INF;
  for (int i = 0; i < N; ++i) {
	for (int j = 0; j < N; ++j) {
	  int k = a[i] + b[j];
	  if (K <= k && k < min_value) {
	    min_value = k;
	  }
	}
  }

  cout << min_value << endl;
}
