package main

import "testing"

func TestMoves(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"abcdef", 609043},
		{"pqrstuv", 1048970},
	}
	for _, c := range cases {
		got := MineCoin(c.in)
		if got != c.want {
			t.Errorf("MineCoin(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}
