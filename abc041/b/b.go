//go:generate echo "https://atcoder.jp/contests/abc041/tasks/abc041_b"
package main

import (
	"bufio"
	"fmt"
	"io"
	"math/big"
	"os"
	"strconv"
)

var in = newScanner(os.Stdin)

func main() {
	A, B, C := newMint(in.Int()), newMint(in.Int()), newMint(in.Int())
	out(A.Mul(B).Mul(C))
}

type mint int

var mod = 1000000007

func setMintMod(x int)   { mod = x }
func newMint(v int) mint { return mint((v%mod + mod) % mod) }
func (x mint) Add(y mint) mint {
	v := x + y
	if v >= mint(mod) {
		return v - mint(mod)
	}
	return v
}
func (x mint) Sub(y mint) mint {
	v := x + mint(mod) - y
	if v >= mint(mod) {
		return v - mint(mod)
	}
	return v
}
func (x mint) Mul(y mint) mint { return (x * y) % mint(mod) }
func (x mint) Pow(p int) mint {
	if p == 0 {
		return mint(1)
	}
	a := x.Pow(p >> 1)
	a = a.Mul(a)
	if p&1 != 0 {
		a = a.Mul(x)
	}
	return a
}
func (x mint) Inverse() mint   { return x.Pow(mod - 2) }
func (x mint) Div(y mint) mint { return x.Mul(y.Inverse()) }
func (x mint) Neg() mint       { return newMint(int(-x)) }

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
func (f *factorial) Get(n int) mint        { return f.fact[n] }
func (f *factorial) GetInverse(n int) mint { return f.factInverse[n] }
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

func out(d ...interface{}) { fmt.Fprintln(os.Stdout, d...) }

func debug(d ...interface{}) { fmt.Fprintln(os.Stderr, d...) }

type scanner struct{ *bufio.Scanner }

const maxBufSize = 600000

func newScanner(r io.Reader) *scanner {
	s := bufio.NewScanner(r)
	s.Buffer(make([]byte, maxBufSize), maxBufSize)
	s.Split(bufio.ScanWords)
	return &scanner{s}
}

func (s *scanner) String() string { s.Scan(); return s.Text() }

func (s *scanner) Strings(n int) []string {
	a := make([]string, n, n)
	for i := range a {
		a[i] = s.String()
	}
	return a
}

func (s *scanner) Int() int { n, _ := strconv.Atoi(s.String()); return n }

func (s *scanner) Ints(n int) []int {
	a := make([]int, n, n)
	for i := range a {
		a[i] = s.Int()
	}
	return a
}

func (s *scanner) Float64() float64 { f, _ := strconv.ParseFloat(s.String(), 64); return f }

func (s *scanner) Float64s(n int) []float64 {
	a := make([]float64, n, n)
	for i := range a {
		a[i] = s.Float64()
	}
	return a
}

func (s *scanner) BigInt() *big.Int { n, _ := new(big.Int).SetString(s.String(), 10); return n }

func (s *scanner) BigInts(n int) []*big.Int {
	a := make([]*big.Int, n, n)
	for i := range a {
		a[i] = s.BigInt()
	}
	return a
}

func iabs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func chmax(x *int, v int) {
	if *x < v {
		*x = v
	}
}

func imax(X ...int) int {
	if len(X) == 0 {
		return 0
	}
	max := X[0]
	for _, x := range X[1:] {
		chmax(&max, x)
	}
	return max
}

func chmin(x *int, v int) {
	if *x > v {
		*x = v
	}
}

func imin(X ...int) int {
	if len(X) == 0 {
		return 0
	}
	min := X[0]
	for _, x := range X[1:] {
		chmin(&min, x)
	}
	return min
}

func isum(X ...int) int {
	s := 0
	for _, x := range X {
		s += x
	}
	return s
}

func ipow(x, n int) int {
	if n == 0 {
		return 1
	}
	a := ipow(x, n>>1)
	a *= a
	if n&1 != 0 {
		a *= x
	}
	return a
}

func digitNum(x int) int {
	d := 0
	for ; x > 0; x /= 10 {
		d++
	}
	return d
}

func digitSum(x int) int {
	s := 0
	for ; x > 0; x /= 10 {
		s += x % 10
	}
	return s
}

func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}

func lcm(a, b int) int { return a * b / gcd(a, b) }

func ifact(x int) int {
	f := 1
	for i := 2; i <= x; i++ {
		f *= i
	}
	return f
}

func perm(X []int) [][]int {
	makeCopy := func(X []int) []int { return append(make([]int, 0, len(X)), X...) }
	n := len(X)
	c := makeCopy(X)
	res := append(make([][]int, 0, ifact(n)), makeCopy(c))
	p := make([]int, n+1, n+1)
	for i := 0; i < n+1; i++ {
		p[i] = i
	}
	for i := 1; i < n; {
		p[i]--
		j := 0
		if i%2 == 1 {
			j = p[i]
		}
		c[i], c[j] = c[j], c[i]
		res = append(res, makeCopy(c))
		for i = 1; p[i] == 0; i++ {
			p[i] = i
		}
	}
	return res
}

func factor(n int) map[int]int {
	m := map[int]int{}
	for i := 2; i*i <= n; i++ {
		for n%i == 0 {
			n /= i
			m[i]++
		}
	}
	if n != 1 {
		m[n]++
	}
	return m
}

func isPrime(n int) bool { f := factor(n); return len(f) == 1 && f[n] == 1 }
