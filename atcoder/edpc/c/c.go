//go:generate echo "https://atcoder.jp/contests/dp/tasks/dp_c"
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

func solve(n int, u [][]int) int {
	t := make([][]int, n+1, n+1)
	for i := range t {
		t[i] = make([]int, 3, 3)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < 3; j++ {
			for k := 0; k < 3; k++ {
				if j == k {
					continue
				}
				if v := t[i][j] + u[i][k]; t[i+1][k] < v {
					t[i+1][k] = v
				}
			}
		}
	}
	max := math.MinInt64
	for _, v := range t[n] {
		if max < v {
			max = v
		}
	}
	return max
}

func main() {
	scan := newScanner(os.Stdin)
	n := scan.Int()
	u := make([][]int, n+1, n+1)
	for i := 0; i <= n; i++ {
		u[i] = []int{scan.Int(), scan.Int(), scan.Int()}
	}
	fmt.Println(solve(n, u))
}
