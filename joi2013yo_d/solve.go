//go:generate echo "https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_d"
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

func chmax(x *int, v int) {
	if *x < v {
		*x = v
	}
}

func iabs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func solve(D, N int, Ts []int, ABCs [][]int) int {
	t := make([][]int, D, D) // t[day_index (0, D-1)][c (0, 200)]
	for i := range t {
		t[i] = make([]int, 201, 201)
	}
	for d := 0; d < D-1; d++ {
		for _, ABC1 := range ABCs {
			if Ts[d] < ABC1[0] || ABC1[1] < Ts[d] {
				continue
			}
			c1 := ABC1[2] // today
			for _, ABC2 := range ABCs {
				if Ts[d+1] < ABC2[0] || ABC2[1] < Ts[d+1] {
					continue
				}
				c2 := ABC2[2] // tomorrow
				chmax(&t[d+1][c2], t[d][c1]+iabs(c2-c1))
			}
		}
	}
	a := 0
	for _, v := range t[D-1] {
		chmax(&a, v)
	}
	return a
}

func main() {
	scan := newScanner(os.Stdin)
	D, N := scan.Int(), scan.Int()
	Ts := scan.Ints(D)
	ABCs := make([][]int, N, N)
	for i := range ABCs {
		ABCs[i] = []int{scan.Int(), scan.Int(), scan.Int()}
	}
	fmt.Println(solve(D, N, Ts, ABCs))
}
