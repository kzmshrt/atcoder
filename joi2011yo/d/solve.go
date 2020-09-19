//go:generate echo "https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d"
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

func solve(N int, nums []int) int {
	t := make([][]int, N-1, N-1) // t[num_index][sum]
	for i := range t {
		t[i] = make([]int, 21, 21)
	}
	t[0][nums[0]] = 1
	for i := 1; i < N-1; i++ {
		for sum := 0; sum <= 20; sum++ {
			if n := sum + nums[i]; n <= 20 {
				t[i][n] += t[i-1][sum]
			}
			if n := sum - nums[i]; 0 <= n {
				t[i][n] += t[i-1][sum]
			}
		}
	}
	return t[N-2][nums[N-1]]
}

func main() {
	scan := newScanner(os.Stdin)
	N := scan.Int()
	nums := scan.Ints(N)
	fmt.Println(solve(N, nums))
}
