//go:generate echo "https://atcoder.jp/contests/abc175/tasks/abc175_b"
package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
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

func solve(N int, L []int) int {
	if N < 3 {
		return 0
	}
	u := []int{}
	m := map[int]int{}
	for _, l := range L {
		if _, ok := m[l]; !ok {
			u = append(u, l)
		}
		m[l]++
	}
	sort.Ints(u)
	a := 0
	for x := 0; x < len(u); x++ {
		for y := x + 1; y < len(u); y++ {
			for z := y + 1; z < len(u); z++ {
				if u[x]+u[y] > u[z] {
					a += m[u[x]] * m[u[y]] * m[u[z]]
				}
			}
		}
	}
	return a
}

func main() {
	scan := newScanner(os.Stdin)
	N := scan.Int()
	L := scan.Ints(N)
	fmt.Println(solve(N, L))
}
