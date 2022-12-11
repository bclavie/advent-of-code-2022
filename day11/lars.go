package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

var file string

var (
	itemsRE = regexp.MustCompile(`Starting items: (.+)`)
	opRE    = regexp.MustCompile(`Operation: new = old (.+)`)
	testRE  = regexp.MustCompile(`Test: divisible by (\d+)`)
	trueRE  = regexp.MustCompile(`If true: throw to monkey (\d+)`)
	falseRE = regexp.MustCompile(`If false: throw to monkey (\d+)`)
)

type Monkey struct {
	Items        []int
	Operation    string
	Divisor      int
	Positive     int
	Negative     int
	InspectCount int
}

func (m *Monkey) AddItem(item int) {
	m.Items = append(m.Items, item)
}

func (m *Monkey) NextItem() (next int) {
	next, m.Items = m.Items[0], m.Items[1:]
	return next
}

func (m *Monkey) Inspect(item int) int {
	m.InspectCount++
	split := strings.Split(m.Operation, " ")
	operator := split[0]
	el := split[1]
	operand := 0
	if el == "old" {
		operand = item
	} else {
		operand, _ = strconv.Atoi(el)
	}
	switch operator {
	case "+":
		return item + operand
	case "*":
		return item * operand
	}
	fmt.Println("Inspection failed")
	return item
}

func (m Monkey) Test(item int) bool {
	return item%m.Divisor == 0
}

func (m Monkey) String() string {
	return fmt.Sprintf("Items: %v\nOperation: old %v\nTest: divisible by %d\n  True: to %d\nFalse: to %d\n",
		m.Items, m.Operation, m.Divisor, m.Positive, m.Negative)
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

func ParseMonkeys(input []string) []Monkey {

	monkeys := []Monkey{}
	monkey := Monkey{}
	for _, line := range input {
		if len(line) < 1 {
			monkeys = append(monkeys, monkey)
			monkey = Monkey{}
		}
		if match := itemsRE.FindStringSubmatch(line); len(match) > 1 {
			items := strings.Split(match[1], ",")
			monkey.Items = make([]int, len(items))
			for i := range items {
				item := strings.TrimSpace(items[i])
				monkey.Items[i], _ = strconv.Atoi(item)
			}
		}
		if match := opRE.FindStringSubmatch(line); len(match) > 1 {
			monkey.Operation = match[1]
		}
		if match := testRE.FindStringSubmatch(line); len(match) > 1 {
			monkey.Divisor, _ = strconv.Atoi(match[1])
		}
		if match := trueRE.FindStringSubmatch(line); len(match) > 1 {
			monkey.Positive, _ = strconv.Atoi(match[1])
		}
		if match := falseRE.FindStringSubmatch(line); len(match) > 1 {
			monkey.Negative, _ = strconv.Atoi(match[1])
		}
	}
	monkeys = append(monkeys, monkey)

	return monkeys
}

func calculateResultA(input []string) int {

	monkeys := ParseMonkeys(input)

	for round := 1; round <= 20; round++ {
		for i := range monkeys {
			for len(monkeys[i].Items) > 0 {
				item := monkeys[i].NextItem()
				item = monkeys[i].Inspect(item)
				item = item / 3
				test := monkeys[i].Test(item)
				if test {
					monkeys[monkeys[i].Positive].AddItem(item)
				} else {
					monkeys[monkeys[i].Negative].AddItem(item)
				}
			}
		}
	}

	counts := make([]int, len(monkeys))
	for i, monkey := range monkeys {
		counts[i] = monkey.InspectCount
	}
	slices.Sort(counts)
	result := counts[len(counts)-1] * counts[len(counts)-2]
	return result
}

func calculateResultB(input []string) int {

	monkeys := ParseMonkeys(input)
	divisorProduct := 1
	for _, monkey := range monkeys {
		divisorProduct *= monkey.Divisor
	}

	for round := 1; round <= 10000; round++ {
		for i := range monkeys {
			for len(monkeys[i].Items) > 0 {
				item := monkeys[i].NextItem()
				item = monkeys[i].Inspect(item)
				// Value is kept effectively the same for all monkeys by this
				item = item % divisorProduct
				test := monkeys[i].Test(item)
				if test {
					monkeys[monkeys[i].Positive].AddItem(item)
				} else {
					monkeys[monkeys[i].Negative].AddItem(item)
				}
			}
		}
	}

	counts := make([]int, len(monkeys))
	for i, monkey := range monkeys {
		counts[i] = monkey.InspectCount
	}
	slices.Sort(counts)
	result := counts[len(counts)-1] * counts[len(counts)-2]
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
