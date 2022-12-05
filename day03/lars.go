package main

import (
	"bufio"
	"fmt"
	"os"

	"golang.org/x/exp/slices"
)

var file string

func GetPriority(r rune) int {
	if 'a' <= r && r <= 'z' {
		return int(r) - 96
	}
	if 'A' <= r && r <= 'Z' {
		return int(r) - 38
	}
	return int(r)
}

func getResult(part string) int {
	input := []string{}

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
		input = append(input, line)
	}

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func calculateResultA(input []string) int {
	result := 0

	for _, line := range input {
		left := []rune(line[:(len(line) / 2)])
		slices.Sort(left)
		right := []rune(line[(len(line) / 2):])
		slices.Sort(right)

		for len(left) > 0 && len(right) > 0 {
			if left[0] == right[0] {
				result += GetPriority(left[0])
				break
			}
			if left[0] < right[0] {
				left = left[1:]
			} else {
				right = right[1:]
			}
		}
	}

	return result
}

func calculateResultB(input []string) int {
	result := 0

	for i := 0; i < len(input); i += 3 {
		one := []rune(input[i])
		slices.Sort(one)
		two := []rune(input[i+1])
		slices.Sort(two)
		three := []rune(input[i+2])
		slices.Sort(three)

		for len(one) > 0 && len(two) > 0 && len(three) > 0 {
			if one[0] == two[0] && one[0] == three[0] {
				result += GetPriority(one[0])
				break
			}
			if one[0] <= two[0] && one[0] <= three[0] {
				one = one[1:]
			} else if two[0] <= one[0] && two[0] <= three[0] {
				two = two[1:]
			} else if three[0] <= one[0] && three[0] <= two[0] {
				three = three[1:]
			}
		}
	}

	return result
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
