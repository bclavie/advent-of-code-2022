package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"golang.org/x/exp/slices"
)

var file string

func getResult(part string) int {
	input := []int{0}

	firstPart := part == "A"

	if file == "" {
		file = "input"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		line := scanner.Text()
		number, err := strconv.Atoi(line)
		if err == nil {
			input[len(input)-1] += number
		} else {
			input = append(input, 0)
		}

	}

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func calculateResultA(input []int) int {
	slices.Sort(input)
	return input[len(input)-1]
}

func calculateResultB(input []int) int {
	slices.Sort(input)
	return input[len(input)-1] + input[len(input)-2] + input[len(input)-3]
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
