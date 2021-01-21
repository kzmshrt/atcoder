//go:generate echo "https://atcoder.jp/contests/abc129/tasks/abc129_c"
package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

var scan = newScanner(os.Stdin)

func solve(n int, b map[int]bool) int {
	t := make([]mint, 100010, 100010)
	t[0] = 1
	for i := 0; i <= n; i++ {
		if !b[i+1] {
			t[i+1] = t[i+1].Add(t[i])
		}
		if !b[i+2] {
			t[i+2] = t[i+2].Add(t[i])
		}
	}
	return int(t[n])
}

func main1() {
	scan := newScanner(os.Stdin)
	n, m := scan.Int(), scan.Int()
	b := map[int]bool{}
	for i := 0; i < m; i++ {
		b[scan.Int()] = true
	}
	fmt.Println(solve(n, b))
}

func solve2(N int, as map[int]bool) int {
	t := make([]mint, N+1, N+1)
	t[0] = newMint(1)
	for i := 1; i < N+1; i++ {
		if 1 < i && !as[i-2] {
			t[i] = t[i].Add(t[i-2])
		}
		if !as[i-1] {
			t[i] = t[i].Add(t[i-1])
		}
	}
	return int(t[N])
}

func main() {
	N, M := scan.Int(), scan.Int()
	as := map[int]bool{}
	for i := 0; i < M; i++ {
		as[scan.Int()] = true
	}
	fmt.Println(solve2(N, as))
}

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

func (s *scanner) IntSlice(l int) []int {
	if l == 0 {
		return []int{}
	}
	sl := make([]int, l, l)
	for i := range sl {
		sl[i] = s.Int()
	}
	return sl
}

func (s *scanner) IntSlice2(l, n int) []int {
	if l == 0 {
		return []int{}
	}
	sl := make([]int, l, l)
	for i := 0; i < n; i++ {
		sl[i] = s.Int()
	}
	return sl
}

const (
	mod = 1000000007
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
