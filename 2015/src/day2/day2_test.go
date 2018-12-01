package main

import "testing"

func TestSmallest(t *testing.T) {
	cases := []struct {
		in, want []int
	}{
		{[]int{2, 3, 4}, []int{2, 3}},
		{[]int{10, 1, 1}, []int{1, 1}},
	}
	for _, c := range cases {
		got := Smallest(c.in)
		if !testSliceEq(got, c.want) {
			t.Errorf("Smallest(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestSlack(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{2, 3, 4}, 6},
		{[]int{10, 1, 1}, 1},
	}
	for _, c := range cases {
		got := Slack(c.in)
		if got != c.want {
			t.Errorf("Slack(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestSurfaceArea(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{2, 3, 4}, 52},
		{[]int{10, 1, 1}, 42},
	}
	for _, c := range cases {
		got := SurfaceArea(c.in)
		if got != c.want {
			t.Errorf("SurfaceArea(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestTotal(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{2, 3, 4}, 58},
		{[]int{10, 1, 1}, 43},
	}
	for _, c := range cases {
		got := Total(c.in)
		if got != c.want {
			t.Errorf("Total(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestVolume(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{2, 3, 4}, 24},
		{[]int{10, 1, 1}, 10},
	}
	for _, c := range cases {
		got := Volume(c.in)
		if got != c.want {
			t.Errorf("Volume(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func TestRibbon(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{2, 3, 4}, 34},
		{[]int{10, 1, 1}, 14},
	}
	for _, c := range cases {
		got := Ribbon(c.in)
		if got != c.want {
			t.Errorf("Volume(%q) == %q, want %q", c.in, got, c.want)
		}
	}
}

func testSliceEq(a, b []int) bool {
	if a == nil && b == nil {
		return true
	}

	if a == nil || b == nil {
		return false
	}

	if len(a) != len(b) {
		return false
	}

	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}
