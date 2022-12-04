package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	bytes, err := os.ReadFile("../data")
	check(err)
	content := string(bytes)
	lines := strings.Split(content, "\n")

	var part1, part2 int
	for _, line := range lines {
		segments := strings.Split(line, ",")
		first := parseRange(segments[0])
		second := parseRange(segments[1])

		if first.contains(second) {
			part1++
		} else if second.contains(first) {
			part1++
		}

		if first.overlaps(second) {
			part2++
		} else if second.overlaps(first) {
			part2++
		}
	}

	fmt.Println("part1", part1)
	fmt.Println("part2", part2)
}

type Range struct {
	start int
	end   int
}

func (r *Range) contains(r2 Range) bool {
	return r.start <= r2.start && r.end >= r2.end
}

func (r *Range) overlaps(r2 Range) bool {
	return r.start <= r2.start && r.end >= r2.start
}

func parseRange(s string) Range {
	segments := strings.Split(s, "-")
	start, _ := strconv.Atoi(segments[0])
	end, _ := strconv.Atoi(segments[1])
	return Range{start, end}
}
