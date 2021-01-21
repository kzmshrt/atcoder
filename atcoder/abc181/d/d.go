//go:generate echo "https://atcoder.jp/contests/abc181/tasks/abc181_d"
package main

import (
	"bufio"
	"fmt"
	"io"
	"math/big"
	"os"
	"strconv"
)

func solve(S string) bool {
	if len(S) == 1 {
		return (S[0]-48)%8 == 0
	}
	if len(S) == 2 {
		n, _ := strconv.Atoi(S)
		r := (n%10)*10 + n/10
		return n%8 == 0 || r%8 == 0
	}
	m := map[int]int{}
	nums := []int{}
	for _, c := range S {
		if n := int(c) - 48; m[n] < 3 {
			m[n]++
			nums = append(nums, n)
		}
		if len(nums) >= 30 {
			break
		}
	}
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			for k := j + 1; k < len(nums); k++ {
				for _, pattern := range perm([]int{nums[i], nums[j], nums[k]}) {
					if (pattern[0]*100+pattern[1]*10+pattern[2])%8 == 0 {
						return true
					}
				}
			}
		}
	}
	return false
}

func main() {
	if S := scan.String(); solve(S) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}

var scan = newScanner(os.Stdin)

// scanner
type scanner struct {
	*bufio.Scanner
}

func newScanner(r io.Reader) *scanner {
	s := bufio.NewScanner(r)
	s.Split(bufio.ScanWords)
	s.Buffer(nil, 100000000)
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
