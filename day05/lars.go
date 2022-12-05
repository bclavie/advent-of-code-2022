package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

var file string

type Stack struct {
	Content []string
}

func (s *Stack) Push(el []string) {
	if s.Content == nil {
		s.Content = el
	} else {
		s.Content = append(s.Content, el...)
	}
}

func (s *Stack) Pop(x int) []string {
	el := s.Content[len(s.Content)-x:]
	s.Content = s.Content[:len(s.Content)-x]
	return el
}

func (s Stack) Top() string {
	return s.Content[len(s.Content)-1]
}

func (s *Stack) Reverse() {
	for i := len(s.Content)/2 - 1; i >= 0; i-- {
		opp := len(s.Content) - 1 - i
		s.Content[i], s.Content[opp] = s.Content[opp], s.Content[i]
	}
}

type Command struct {
	Move int
	From int
	To   int
}

func ParseCommand(line string) Command {
	re := regexp.MustCompile(`move (\d+) from (\d+) to (\d+)`)
	matches := re.FindStringSubmatch(line)
	move, _ := strconv.Atoi(matches[1])
	from, _ := strconv.Atoi(matches[2])
	to, _ := strconv.Atoi(matches[3])
	return Command{move, from, to}
}

func getResult(part string) string {
	commands := []Command{}

	firstPart := part == "A"

	if file == "" {
		file = "input"
	}
	f, _ := os.Open(file)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	stacks := []Stack{}
	buildStacks := true
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			continue
		}
		if string(line[1]) == "1" {
			buildStacks = false
			continue
		}
		if buildStacks {
			numStacks := (len(line) + 1) / 4
			for i := 0; i < numStacks; i++ {
				if len(stacks) < i+1 {
					stacks = append(stacks, Stack{})
				}
				el := string(line[4*i+1])
				if el != " " {
					stacks[i].Push([]string{el})
				}
			}
		} else {
			commands = append(commands, ParseCommand(line))
		}
	}

	for i := range stacks {
		stacks[i].Reverse()
	}

	if firstPart {
		return calculateResultA(stacks, commands)
	}

	return calculateResultB(stacks, commands)
}

func calculateResultA(stacks []Stack, commands []Command) string {
	for _, command := range commands {
		for i := 0; i < command.Move; i++ {
			el := stacks[command.From-1].Pop(1)
			stacks[command.To-1].Push(el)
		}
	}

	result := ""
	for _, stack := range stacks {
		result += stack.Top()
	}

	return result
}

func calculateResultB(stacks []Stack, commands []Command) string {
	for _, command := range commands {
		el := stacks[command.From-1].Pop(command.Move)
		stacks[command.To-1].Push(el)
	}

	result := ""
	for _, stack := range stacks {
		result += stack.Top()
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
