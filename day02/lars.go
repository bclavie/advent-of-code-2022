package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var file string

type Round struct {
	Opp    string
	Self   string
	Result string
}

func ResultScore(result string) int {
	switch result {
	// Rock
	case "X":
		return 0
	// Paper
	case "Y":
		return 3
	// Sciccors
	case "Z":
		return 6
	default:
		return 0
	}
}

// ShapeScore returns the value of the shape needed for solving the match
// For getting the actual shape score for the result just add one
func ShapeScore(shape string) int {
	switch shape {
	// Rock
	case "A", "X":
		return 0
	// Paper
	case "B", "Y":
		return 1
	// Sciccors
	case "C", "Z":
		return 2
	default:
		return -1
	}
}

func getResult(part string) int {
	input := []Round{}

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
		split := strings.Split(line, " ")
		round := Round{Opp: split[0]}
		if firstPart {
			round.Self = split[1]
		} else {
			round.Result = split[1]
		}
		input = append(input, round)
	}

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func calculateResultA(input []Round) int {
	result := 0
	for _, r := range input {
		selfScore := ShapeScore(r.Self)
		oppScore := ShapeScore(r.Opp)
		// Default loss
		r.Result = "X"
		// Draw
		if oppScore == selfScore {
			r.Result = "Y"
		}
		// Win - own move is one higher than opp move
		if (oppScore+1)%3 == selfScore {
			r.Result = "Z"
		}

		result += selfScore + 1 + ResultScore(r.Result)
	}
	return result
}

func calculateResultB(input []Round) int {
	result := 0
	for _, r := range input {
		oppScore := ShapeScore(r.Opp)
		shapeScore := 0
		switch r.Result {
		case "X":
			// Shape before is the one two later -> prevents -1 % 3
			shapeScore = (oppScore + 2) % 3
		case "Y":
			shapeScore = oppScore
		case "Z":
			shapeScore = (oppScore + 1) % 3
		}
		result += ResultScore(r.Result) + shapeScore + 1
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
