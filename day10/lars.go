package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"

	"golang.org/x/exp/slices"
)

var file string

var (
	noopRE = regexp.MustCompile(`noop`)
	addxRE = regexp.MustCompile(`addx (-?\d+)`)
)

type Clock struct {
	Cycle        int
	X            int
	Sum          int
	TargetCycles []int
	Rows         [6][40]bool
}

func (c *Clock) Step() {
	c.Cycle++
	// Stuff that happens during the cycle
	// A
	if slices.Contains(c.TargetCycles, c.Cycle) {
		c.Sum += c.X * c.Cycle
	}
	// B
	x := (c.Cycle - 1) % 40
	y := (c.Cycle - 1) / 40
	if c.X-1 <= x && x <= c.X+1 {
		c.Rows[y][x] = true
	}
}

func (c Clock) String() string {
	text := ""
	for _, row := range c.Rows {
		for _, cell := range row {
			if cell {
				text += "#"
			} else {
				text += "."
			}
		}
		text += "\n"
	}
	return text
}

func getResult(part string) string {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return fmt.Sprintf("%d", calculateResultA(input))
	}

	return calculateResultB(input)
}

func getInput() []string {
	input := []string{}

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

	return input
}

func calculateResultA(input []string) int {

	targetCycles := []int{20, 60, 100, 140, 180, 220}
	clock := Clock{0, 1, 0, targetCycles, [6][40]bool{}}
	i := 0
	for i < len(input) {
		clock.Step()
		if addxRE.MatchString(input[i]) {
			matches := addxRE.FindStringSubmatch(input[i])
			value, _ := strconv.Atoi(matches[1])
			clock.Step()
			clock.X += value
		}
		i++
	}

	return clock.Sum
}

func calculateResultB(input []string) string {

	clock := Clock{0, 1, 0, []int{}, [6][40]bool{}}
	i := 0
	for i < len(input) {
		clock.Step()
		if addxRE.MatchString(input[i]) {
			matches := addxRE.FindStringSubmatch(input[i])
			value, _ := strconv.Atoi(matches[1])
			clock.Step()
			clock.X += value
		}
		i++
	}

	return clock.String()
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
