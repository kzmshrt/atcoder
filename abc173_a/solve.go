//go:generate echo "https://atcoder.jp/contests/abc173/tasks/abc173_a"
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

func solve(N int) int {
	return (1000 - N%1000) % 1000
}

func run(r io.Reader, w io.Writer) {
	scan := newScanner(r)
	fmt.Fprintln(w, solve(scan.Int()))
}

func main() {
	run(os.Stdin, os.Stdout)
}
