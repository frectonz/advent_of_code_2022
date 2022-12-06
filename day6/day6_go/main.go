package main

import (
	"fmt"
	"os"
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
	line := lines[0]

	part1Count := count_unique(line, 4)
	fmt.Println("Part 1: ", part1Count)
	part2Count := count_unique(line, 14)
	fmt.Println("Part 2: ", part2Count)
}
func count_unique(line string, size int) int {
	for i := range line {
		set := make_map(line, i, size)
		count := count(set)

		if count == size {
			return i + size
		}
	}
	return 0
}

func safe_get(line string, index int) byte {
	if index < len(line) {
		return line[index]
	}
	return 0
}

func make_map(line string, start int, size int) map[byte]bool {
	set := make(map[byte]bool)
	for j := start; j < start+size; j++ {
		element := safe_get(line, j)
		set[element] = true
	}
	return set
}

func count(set map[byte]bool) int {
	var count int
	for _, v := range set {
		if v {
			count++
		}
	}
	return count
}
