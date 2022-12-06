package main

import (
	"fmt"
	"os"
)

var file string

func getResult(part string) int {
	firstPart := part == "A"

	if file == "" {
		file = "input"
	}
	bytes, _ := os.ReadFile(file)

	if firstPart {
		return calculateResult(string(bytes), 4)
	}

	return calculateResult(string(bytes), 14)
}

func calculateResult(input string, threshold int) int {

	seen := make(map[string]int)
	i := 0
	cutoff := threshold - 1
	for i < len(input) {
		// Add current position
		seen[string(input[i])] = i
		if i >= cutoff {
			// Check for win condition
			if len(seen) == threshold {
				return i + 1
			}
			// Remove value thats out of range next iteration if it was not duplicated since
			if x := seen[string(input[i-cutoff])]; x == i-cutoff {
				delete(seen, string(input[i-cutoff]))
			}
		}
		i++
	}

	return -1
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
