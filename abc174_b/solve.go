//go:generate echo "https://atcoder.jp/contests/abc174/tasks/abc174_b"
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

func (s *scanner) Float64() float64 {
	s.Scan()
	f, _ := strconv.ParseFloat(s.Text(), 64)
	return f
}

func (s *scanner) Floats64(l int) []float64 {
	if l == 0 {
		return []float64{}
	}
	sl := make([]float64, l, l)
	for i := range sl {
		sl[i] = s.Float64()
	}
	return sl
}

func solve(D float64, ps [][]float64) (c int) {
	for _, p := range ps {
		if math.Sqrt(math.Pow(p[0], 2)+math.Pow(p[1], 2)) <= D {
			c++
		}
	}
	return c
}

func main() {
	scan := newScanner(os.Stdin)
	N, D := scan.Int(), scan.Float64()
	ps := make([][]float64, N, N)
	for i := range ps {
		ps[i] = []float64{scan.Float64(), scan.Float64()}
	}
	fmt.Println(solve(D, ps))
}
