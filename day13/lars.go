package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"golang.org/x/exp/slices"
)

var file string

type Pair struct {
	Left  Item
	Right Item
}

type Item struct {
	Value  int
	List   []*Item
	Parent *Item
}

func (i Item) IsValue() bool {
	return i.Value > -1
}

func (i Item) String() string {
	text := ""
	if i.Value > -1 {
		return fmt.Sprint(i.Value)
	} else {
		text += "["
		for index, el := range i.List {
			text += el.String()
			if index < len(i.List)-1 {
				text += ","
			}
		}
		text += "]"
	}
	return text
}

// Compare returns <0 value if left is lower, 0 if equal and >0 if right is lower
func (left *Item) Compare(right *Item) int {
	if left.IsValue() && right.IsValue() {
		return left.Value - right.Value
	}
	if left.IsValue() && !right.IsValue() {
		standin := &Item{Value: -1, List: []*Item{left}}
		return standin.Compare(right)
	}
	if !left.IsValue() && right.IsValue() {
		standin := &Item{Value: -1, List: []*Item{right}}
		return left.Compare(standin)
	}
	if !left.IsValue() && !right.IsValue() {
		lastComparison := 0
		for i := 0; lastComparison == 0; i++ {
			// Left list ran out of elements
			if i > len(left.List)-1 && i <= len(right.List)-1 {
				return -1
			}
			// Right list ran out of elements
			if i <= len(left.List)-1 && i > len(right.List)-1 {
				return 1
			}
			// Both ran out of elements
			if i > len(left.List)-1 && i > len(right.List)-1 {
				return 0
			}
			lastComparison = left.List[i].Compare(right.List[i])
		}
		return lastComparison
	}
	return 0
}

func ParseItem(line string) Item {
	outer := Item{Value: -1, List: []*Item{}}
	// Loop over everything but the first and last chars because they are "[" and "]"
	current := &outer
	for i := 1; i < len(line)-1; i++ {
		char := string(line[i])
		if _, err := strconv.Atoi(char); err == nil {
			fullNumberString := char
			for !slices.Contains([]string{"[", "]", ","}, string(line[i+1])) {
				i++
				fullNumberString += string(line[i])
			}
			fullValue, _ := strconv.Atoi(fullNumberString)
			current.List = append(current.List, &Item{Value: fullValue, Parent: current})
		} else if char == "[" {
			newList := Item{Value: -1, List: []*Item{}, Parent: current}
			current.List = append(current.List, &newList)
			current = &newList
		} else if char == "]" {
			current = current.Parent
		}
	}

	return outer
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func getInput() []string {
	input := []string{}

	if file == "" {
		file = "input.txt"
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

	pairs := []Pair{}
	for i := 0; i < len(input)-1; i += 3 {
		pair := Pair{
			Left:  ParseItem(input[i]),
			Right: ParseItem(input[i+1]),
		}
		pairs = append(pairs, pair)
	}

	for _, pair := range pairs {
		fmt.Println(pair.Left)
		fmt.Println(pair.Right)
		fmt.Println()
	}

	result := 0
	for i, pair := range pairs {
		if pair.Left.Compare(&pair.Right) < 0 {
			result += i + 1
		}
	}

	return result
}

func calculateResultB(input []string) int {

	Divider1 := &Item{Value: -1, List: []*Item{{Value: -1, List: []*Item{{Value: 2}}}}}
	Divider2 := &Item{Value: -1, List: []*Item{{Value: -1, List: []*Item{{Value: 6}}}}}
	packets := []*Item{Divider1, Divider2}
	for _, line := range input {
		if len(line) > 0 {
			newItem := ParseItem(line)
			packets = append(packets, &newItem)
		}
	}

	slices.SortFunc(packets, func(left, right *Item) bool {
		return left.Compare(right) < 0
	})

	div1Pos := 0
	div2Pos := 0
	for i, item := range packets {
		if item == Divider1 {
			div1Pos = i + 1
		}
		if item == Divider2 {
			div2Pos = i + 1
		}
	}

	return div1Pos * div2Pos

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
