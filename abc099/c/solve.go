//go:generate echo "https://atcoder.jp/contests/abc099/tasks/abc099_c"
package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
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

func solve(n int) int {
	t := make([]int, n+1, n+1)
	for i := range t {
		t[i] = math.MaxInt64
	}
	t[0] = 0
	for i := 0; i < n; i++ {
		if v := t[i] + 1; v < t[i+1] {
			t[i+1] = v
		}
		for _, k := range []int{6, 9} {
			j := 1
			p := i + ipow(k, j)
			for p <= n {
				if v := t[i] + 1; v < t[p] {
					t[p] = v
				}
				j++
				p = i + ipow(k, j)
			}
		}
	}
	return t[n]
}

func main() {
	scan := newScanner(os.Stdin)
	n := scan.Int()
	fmt.Println(solve(n))
}
