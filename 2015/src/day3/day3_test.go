package main

import "testing"

func TestMoves(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{">", 2},
		{"^v^v^v^v^v^v", 2},
		{"^>v<", 4},
		{input, 2592},
	}
	for _, c := range cases {
		got := Moves(c.in)
		if got != c.want {
			t.Errorf("Moves(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestRoboMoves(t *testing.T) {
	cases := []struct {
		in   string
		want int
	}{
		{"^v", 3},
		{"^>v<", 3},
		{"^v^v^v^v^v", 11},
	}
	for _, c := range cases {
		got := RoboMoves(c.in)
		if got != c.want {
			t.Errorf("Moves(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}
