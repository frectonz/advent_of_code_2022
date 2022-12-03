package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func part1() {
	bytes, err := os.ReadFile("../data")
	check(err)

	content := string(bytes)
	lines := strings.Split(content, "\n")

	var sum int
	for _, line := range lines {
		half_point := len(line) / 2

		first_half := line[:half_point]
		second_half := line[half_point:]

		for _, c := range first_half {
			if strings.Contains(second_half, string(c)) {
				sum += charToPriority(c)
				break
			}
		}
	}

	fmt.Println("part 1", sum)
}

func part2() {
	bytes, err := os.ReadFile("../data")
	check(err)

	content := string(bytes)
	lines := strings.Split(content, "\n")

	var sum int
	for i := 0; i < len(lines)/3; i++ {
		idx := i * 3
		one := lines[idx]
		two := lines[idx+1]
		three := lines[idx+2]

		for _, c := range one {
			string_c := string(c)
			if strings.Contains(two, string_c) && strings.Contains(three, string_c) {
				sum += charToPriority(c)
				break
			}
		}
	}

	fmt.Println("part 2", sum)
}

func charToPriority(c rune) int {
	if unicode.IsUpper(c) {
		return int(c) - 38
	} else {
		return int(c) - 96
	}
}

func main() {
	part1()
	part2()
}
