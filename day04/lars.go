package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var file string

type Range struct {
	Min int
	Max int
}

type Pair struct {
	A Range
	B Range
}

func LineToPair(line string) Pair {
	pairSplit := strings.Split(line, ",")
	aRange := strings.Split(pairSplit[0], "-")
	aMin, _ := strconv.Atoi(aRange[0])
	aMax, _ := strconv.Atoi(aRange[1])
	bRange := strings.Split(pairSplit[1], "-")
	bMin, _ := strconv.Atoi(bRange[0])
	bMax, _ := strconv.Atoi(bRange[1])
	return Pair{Range{aMin, aMax}, Range{bMin, bMax}}
}

func getResult(part string) int {
	input := []Pair{}

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
		input = append(input, LineToPair(line))
	}

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func calculateResultA(input []Pair) int {
	result := 0
	for _, pair := range input {
		if pair.A.Min >= pair.B.Min && pair.A.Max <= pair.B.Max {
			result++
		} else if pair.B.Min >= pair.A.Min && pair.B.Max <= pair.A.Max {
			result++
		}
	}
	return result
}

func calculateResultB(input []Pair) int {
	result := 0
	for _, pair := range input {
		if pair.B.Min <= pair.A.Min && pair.A.Min <= pair.B.Max {
			result++
			continue
		}
		if pair.B.Min <= pair.A.Max && pair.A.Max <= pair.B.Max {
			result++
			continue
		}
		if pair.A.Min <= pair.B.Min && pair.B.Min <= pair.A.Max {
			result++
			continue
		}
		if pair.A.Min <= pair.B.Max && pair.B.Max <= pair.A.Max {
			result++
			continue
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
