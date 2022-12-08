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
	input := string(bytes)
	lines := strings.Split(input, "\n")
	intLines := make([][]int, len(lines))

	for i, line := range lines {
		intLine := make([]int, len(line))
		for j, digit := range strings.Split(line, "") {
			intDigit, err := strconv.Atoi(digit)
			check(err)
			intLine[j] = intDigit
		}
		intLines[i] = intLine
	}

	trees := make([]Tree, len(intLines[0])*len(intLines))

	for row, line := range intLines {
		for col, treeHeight := range line {
			topToEdge := make([]int, 0)
			for i := 1; i <= row; i++ {
				topToEdge = append(topToEdge, intLines[row-i][col])
			}

			bottomToEdge := make([]int, 0)
			for i := 1; i <= len(intLines)-row-1; i++ {
				bottomToEdge = append(bottomToEdge, intLines[row+i][col])
			}

			leftToEdge := make([]int, 0)
			for i := 1; i <= col; i++ {
				leftToEdge = append(leftToEdge, intLines[row][col-i])
			}

			rightToEdge := make([]int, 0)
			for i := 1; i <= len(intLines[row])-col-1; i++ {
				rightToEdge = append(rightToEdge, intLines[row][col+i])
			}

			trees[row*len(intLines)+col] = Tree{
				value:        treeHeight,
				topToEdge:    topToEdge,
				leftToEdge:   leftToEdge,
				rightToEdge:  rightToEdge,
				bottomToEdge: bottomToEdge,
			}
		}
	}

	visibleTrees := 0
	for _, tree := range trees {
		if tree.IsVisible() {
			visibleTrees += 1
		}
	}

	maxScenicScore := 0
	for _, tree := range trees {
		scenicScore := tree.ScenicScore()
		if scenicScore > maxScenicScore {
			maxScenicScore = scenicScore
		}
	}

	fmt.Println("Part 1", visibleTrees)
	fmt.Println("Part 2", maxScenicScore)
}

type Tree struct {
	value        int
	topToEdge    []int
	leftToEdge   []int
	rightToEdge  []int
	bottomToEdge []int
}

func (t Tree) IsVisible() bool {
	visibleOnTop := isVisible(t.value, t.topToEdge)
	visibleOnBottom := isVisible(t.value, t.bottomToEdge)
	visibleOnLeft := isVisible(t.value, t.leftToEdge)
	visibleOnRight := isVisible(t.value, t.rightToEdge)

	return visibleOnTop || visibleOnBottom || visibleOnLeft || visibleOnRight
}

func isVisible(value int, ints []int) bool {
	var count int
	for _, tree := range ints {
		if tree < value {
			count += 1
		}
	}
	return count == len(ints)
}

func (t Tree) ScenicScore() int {
	topView := countVisibleTrees(t.value, t.topToEdge)
	bottomView := countVisibleTrees(t.value, t.bottomToEdge)
	leftView := countVisibleTrees(t.value, t.leftToEdge)
	rightView := countVisibleTrees(t.value, t.rightToEdge)

	return topView * bottomView * leftView * rightView
}

func countVisibleTrees(value int, ints []int) int {
	var count int
	var blocked bool
	for _, tree := range ints {
		if !blocked {
			count += 1
			if tree >= value {
				blocked = true
			} else {
				blocked = false
			}
		}
	}
	return count
}
