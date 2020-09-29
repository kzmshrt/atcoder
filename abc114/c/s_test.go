package main

import (
	"reflect"
	"testing"
)

func TestEnumeratePatterns(t *testing.T) {
	type args struct {
		X []int
		d int
	}
	tests := []struct {
		name string
		args args
		want [][][]int
	}{
		{"2 numbers, 3 digits", args{[]int{3, 5}, 2}, [][][]int{{{3}, {5}}, {{3, 3}, {3, 5}, {5, 3}, {5, 5}}}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := EnumeratePatterns(tt.args.X, tt.args.d); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("EnumeratePatterns() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestIntSliceToInt(t *testing.T) {
	type args struct {
		X []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{"a number", args{[]int{1, 2, 3, 5, 7}}, 12357},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := IntSliceToInt(tt.args.X); got != tt.want {
				t.Errorf("IntSliceToInt() = %v, want %v", got, tt.want)
			}
		})
	}
}
