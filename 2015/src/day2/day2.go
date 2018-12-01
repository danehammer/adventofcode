package main

import "sort"

// Smallest Find the smallest dimension
func Smallest(dims []int) []int {
	sort.Ints(dims)
	return dims[0:2]
}

// Slack compute the slack required
func Slack(dims []int) int {
	smallest := Smallest(dims)
	return smallest[0] * smallest[1]
}

// SurfaceArea is 2*l*w + 2*w*h + 2*h*l
func SurfaceArea(dims []int) int {
	return 2*dims[0]*dims[1] + 2*dims[1]*dims[2] + 2*dims[2]*dims[0]
}

// Total the amount needed for one present
func Total(dims []int) int {
	slack := Slack(dims)
	surfaceArea := SurfaceArea(dims)
	return surfaceArea + slack
}

// Volume is l*w*h
func Volume(dims []int) int {
	return dims[0] * dims[1] * dims[2]
}

// Ribbon is 2*(Smallest dims) + Volume
func Ribbon(dims []int) int {
	smalls := Smallest(dims)
	ribbon := 2*smalls[0] + 2*smalls[1]
	bow := Volume(dims)
	return ribbon + bow
}
