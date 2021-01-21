//go:generate echo "https://atcoder.jp/contests/abc097/tasks/arc097_a"
package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"io"
	"math/big"
	"os"
	"sort"
	"strconv"
)

var scan = newScanner(os.Stdin)

// TLE
func solve1(s string, K int) string {
	m := map[string]struct{}{}
	h := new(Heap)
	for l := 0; l < len(s); l++ {
		for r := l + 1; r <= len(s); r++ {
			if _, ok := m[s[l:r]]; !ok {
				m[s[l:r]] = struct{}{}
				heap.Push(h, s[l:r])
			}
		}
	}
	for i := 0; i < K-1; i++ {
		heap.Pop(h)
	}
	return heap.Pop(h).(string)
}

// TLE
func solve2(s string, K int) string {
	p := []string{}
	for n := 1; n <= len(s); n++ {
		for i := 0; i < len(s)-n+1; i++ {
			p = append(p, s[i:i+n])
		}
	}
	sort.Strings(p)
	m := map[string]struct{}{}
	k := 0
	for _, v := range p {
		if _, ok := m[v]; !ok {
			m[v] = struct{}{}
			k++
			if k == K {
				return v
			}
		}
	}
	return ""
}

func solve3(s string, K int) string {
	p := []string{}
	m := map[string]struct{}{}
	k := 0
	for c := 'a'; c <= 'z'; c++ {
		for i, cs := range s {
			if c != cs {
				continue
			}
			for j := i; j < len(s); j++ {
				for q := j; q <= len(s); q++ {
					if _, ok := m[s[j:q]]; !ok {
						m[s[j:q]] = struct{}{}
						p = append(p, s[j:q])
						k++
					}
				}
			}
		}
		if K <= k {
			break
		}
	}
	sort.Strings(p)
	return p[K]
}

func solve4(s string, K int) string {
	N := len(s)
	m := map[string]struct{}{}
	p := []string{}
	for n := 1; n <= imin(N, 5); n++ {
		for i := 0; i <= N-n; i++ {
			if _, ok := m[s[i:i+n]]; !ok {
				m[s[i:i+n]] = struct{}{}
				p = append(p, s[i:i+n])
			}
		}
	}
	sort.Strings(p)
	return p[K-1]
}

func main() {
	s, K := scan.String(), scan.Int()
	fmt.Println(solve4(s, K))
}

type Heap []string

func (h Heap) Len() int            { return len(h) }
func (h Heap) Less(i, j int) bool  { return h[i] < h[j] }
func (h Heap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *Heap) Push(x interface{}) { *h = append(*h, x.(string)) }
func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type scanner struct {
	*bufio.Scanner
}

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

func lcm(a, b int) int {
	return a * b / gcd(a, b)
}

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

func isPrime(n int) bool {
	f := factor(n)
	return len(f) == 1 && f[n] == 1
}
