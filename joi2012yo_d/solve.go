//go:generate echo "https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_d"
package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

type scanner struct{ *bufio.Scanner }

func newScanner(r io.Reader) *scanner {
	s := bufio.NewScanner(r)
	s.Split(bufio.ScanWords)
	s.Buffer(nil, 100000000)
	return &scanner{s}
}

func (s *scanner) Int() int {
	s.Scan()
	n, _ := strconv.Atoi(s.Text())
	return n
}

func (s *scanner) Ints(l int) []int {
	if l == 0 {
		return []int{}
	}
	sl := make([]int, l, l)
	for i := range sl {
		sl[i] = s.Int()
	}
	return sl
}

const (
	mod = 10000
)

type mint int

func newMint(v int) mint {
	return mint((v%mod + mod) % mod)
}

func (x mint) Add(y mint) mint {
	v := x + y
	if v >= mod {
		return v - mod
	}
	return v
}

func (x mint) Sub(y mint) mint {
	v := x + mod - y
	if v >= mod {
		return v - mod
	}
	return v
}

func (x mint) Mul(y mint) mint {
	return (x * y) % mod
}

func (x mint) pow(n int) mint {
	if n == 0 {
		return mint(1)
	}
	a := x.pow(n >> 1)
	a = a.Mul(a)
	if n&1 != 0 {
		a = a.Mul(x)
	}
	return a
}

func (x mint) Inverse() mint {
	return x.pow(mod - 2)
}

func (x mint) Div(y mint) mint {
	return x * y.Inverse()
}

func (x mint) Neg() mint {
	return newMint(int(-x))
}

type factorial struct {
	fact        []mint
	factInverse []mint
}

func newFactorial(n int) *factorial {
	f := new(factorial)
	f.init(n)
	return f
}

func (f *factorial) init(n int) {
	fact := make([]mint, n+1, n+1)
	fact[0] = newMint(1)
	for i := 1; i <= n; i++ {
		fact[i] = fact[i-1].Mul(newMint(i))
	}

	inv := make([]mint, n+1, n+1)
	inv[n] = fact[n].Inverse()
	for i := n; i >= 1; i-- {
		inv[i-1] = inv[i].Mul(newMint(i))
	}

	f.fact = fact
	f.factInverse = inv
}

func (f *factorial) Get(n int) mint {
	return f.fact[n]
}

func (f *factorial) GetInverse(n int) mint {
	return f.factInverse[n]
}

func (f *factorial) Permutation(n, k int) mint {
	if k < 0 || n < k {
		return 0
	}
	return f.fact[n].Mul(f.factInverse[n-k])
}

func (f *factorial) Combination(n, k int) mint {
	if k < 0 || n < k {
		return 0
	}
	return f.fact[n].Mul(f.factInverse[k]).Mul(f.factInverse[n-k])
}

func solve(N, K int, ABs map[int]int) int {
	// t[day(0, N)][pasta_2_day_before (0, 3)][pasta_1_day_before (0, 3)] = pattern_num
	t := make([][][]mint, N+1, N+1)
	for i := range t {
		t[i] = make([][]mint, 4, 4)
		for j := range t[i] {
			t[i][j] = make([]mint, 4, 4)
		}
	}

	t[0][0][0] = 1

	for d := 0; d < N; d++ { // day
		for i := 0; i < 4; i++ { // pasta 2 day before
			for j := 0; j < 4; j++ { // pasta 1 day before
				for k := 1; k < 4; k++ { // pasta today
					if menu, ok := ABs[d]; ok && k != menu {
						continue
					}
					if i == j && j == k {
						continue
					}
					t[d+1][j][k] = t[d+1][j][k].Add(t[d][i][j])
				}
			}
		}
	}

	sum := newMint(0)
	for i := 0; i < 4; i++ {
		for j := 0; j < 4; j++ {
			sum = sum.Add(t[N][i][j])
		}
	}
	return int(sum)
}

func main() {
	scan := newScanner(os.Stdin)
	N, K := scan.Int(), scan.Int()
	ABs := map[int]int{}
	for i := 0; i < K; i++ {
		ABs[scan.Int()-1] = scan.Int()
	}
	fmt.Println(solve(N, K, ABs))
}
