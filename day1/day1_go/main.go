package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	data, err := os.ReadFile("../data")
	check(err)

	string_data := string(data)
	elfCaloriesUnparsed := strings.Split(string_data, "\n\n")

	elfCalories := make([]int, len(elfCaloriesUnparsed))
	for i, e := range elfCaloriesUnparsed {
		elfCalories[i] = sumElfCalories(e)
	}
	sort.Ints(elfCalories)

	sum := 0
	for i := len(elfCalories) - 1; i > len(elfCalories)-4; i-- {
		fmt.Println(i-len(elfCalories), elfCalories[i])
		sum += elfCalories[i]
	}
	println("sum", sum)
}

func sumElfCalories(s string) int {
	calories := strings.Split(s, "\n")
	sum := 0
	for _, c := range calories {
		cInt, err := strconv.Atoi(c)
		check(err)
		sum += cInt
	}
	return sum
}
