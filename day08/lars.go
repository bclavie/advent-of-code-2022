package main

import (
	"bufio"
	"fmt"
	"os"

	"golang.org/x/exp/slices"
)

var file string

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() [][]int {
	// rows [y][x]
	rows := [][]int{}

	if file == "" {
		file = "input"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		row := make([]int, len(line))
		for i, char := range line {
			row[i] = int(char - '0')
		}
		rows = append(rows, row)
	}

	return rows
}

func calculateResultA(rows [][]int) int {
	result := 0
	for y, row := range rows {
		for x := range row {
			visible := IsTreeVisible(rows, x, y)
			result += visible
		}
	}

	return result
}

func IsTreeVisible(rows [][]int, x, y int) int {
	// Skip edge trees
	if x == 0 || y == 0 || x == len(rows)-1 || y == len(rows)-1 {
		return 1
	}
	tree := rows[y][x]
	// Check top of tree -> <y =x
	topTrees := []int{}
	for i := 0; i < y; i++ {
		topTrees = append(topTrees, rows[i][x])
	}
	slices.Sort(topTrees)
	if tree > topTrees[len(topTrees)-1] {
		return 1
	}
	// Check bottom of tree -> >y =x
	bottomTrees := []int{}
	for i := y + 1; i < len(rows); i++ {
		bottomTrees = append(bottomTrees, rows[i][x])
	}
	slices.Sort(bottomTrees)
	if tree > bottomTrees[len(bottomTrees)-1] {
		return 1
	}
	// Check left of tree -> =y <x
	leftTrees := []int{}
	for i := 0; i < x; i++ {
		leftTrees = append(leftTrees, rows[y][i])
	}
	slices.Sort(leftTrees)
	if tree > leftTrees[len(leftTrees)-1] {
		return 1
	}
	// Check left of tree -> =y <x
	rightTrees := []int{}
	for i := x + 1; i < len(rows); i++ {
		rightTrees = append(rightTrees, rows[y][i])
	}
	slices.Sort(rightTrees)
	if tree > rightTrees[len(rightTrees)-1] {
		return 1
	}

	return 0
}

func calculateResultB(rows [][]int) int {

	result := 0
	for y, row := range rows {
		for x := range row {
			score := ScenicScore(rows, x, y)
			if score > result {
				result = score
			}
		}
	}

	return result
}

func ScenicScore(rows [][]int, x, y int) int {
	tree := rows[y][x]
	// Check top of tree -> <y =x
	topTrees := 0
	for i := y - 1; i >= 0; i-- {
		topTrees++
		if tree <= rows[i][x] {
			break
		}
	}
	// Check bottom of tree -> >y =x
	bottomTrees := 0
	for i := y + 1; i < len(rows); i++ {
		bottomTrees++
		if tree <= rows[i][x] {
			break
		}
	}
	// Check left of tree -> =y <x
	leftTrees := 0
	for i := x - 1; i >= 0; i-- {
		leftTrees++
		if tree <= rows[y][i] {
			break
		}
	}
	// Check left of tree -> =y <x
	rightTrees := 0
	for i := x + 1; i < len(rows); i++ {
		rightTrees++
		if tree <= rows[y][i] {
			break
		}
	}

	return topTrees * bottomTrees * leftTrees * rightTrees
}

func main() {
	argsWithProg := os.Args

	var part string
	if len(argsWithProg) < 2 {
		part = "A"
	} else {
		part = argsWithProg[1]
	}

	fmt.Println(getResult(part))
}
