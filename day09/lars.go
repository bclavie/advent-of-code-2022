package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
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

type Point struct {
	X           int
	Y           int
	TailVisited bool
}

type Knot struct {
	Name     string
	Position *Point
	Next     *Knot
}

type Grid struct {
	Start   *Point
	Head    *Knot
	Tail    *Knot
	XOffset int
	YOffset int
	Columns [][]*Point
}

func (g Grid) String() string {
	text := ""
	for y := len(g.Columns[0]) - 1; y > -1; y-- {
		for x := 0; x < len(g.Columns); x++ {
			point := g.Columns[x][y]
			foundKnot := false
			current := g.Head
			for current != nil {
				if current.Position == point {
					text += current.Name
					foundKnot = true
					break
				}
				current = current.Next
			}
			if !foundKnot {
				if g.Start == point {
					text += "s"
				} else if g.Columns[x][y].TailVisited {
					text += "#"
				} else {
					text += "."
				}
			}
		}
		text += "\n"
	}
	return text
}

func (g Grid) TailVisited() int {
	result := 0
	for x := 0; x < len(g.Columns); x++ {
		for y := 0; y < len(g.Columns[x]); y++ {
			if g.Columns[x][y].TailVisited {
				result++
			}
		}
	}
	return result
}

func (g Grid) OutOfBounds(x, y int) bool {
	minPoint := g.Columns[0][0]
	if x < minPoint.X || y < minPoint.Y {
		return true
	}
	maxPoint := g.Columns[len(g.Columns)-1][len(g.Columns[0])-1]
	if x > maxPoint.X || y > maxPoint.Y {
		return true
	}
	return false
}

func (g *Grid) ExtendGrid(newX, newY int) {
	minPoint := g.Columns[0][0]
	if newX < minPoint.X {
		g.XOffset--
		g.Columns = append([][]*Point{make([]*Point, len(g.Columns[0]))}, g.Columns...)
		g.InitializeColumn(newX)
	}
	if newY < minPoint.Y {
		g.YOffset--
		for x := g.XOffset; x < len(g.Columns)+g.XOffset; x++ {
			index := x - g.XOffset
			g.Columns[index] = append([]*Point{{x, newY, false}}, g.Columns[index]...)
		}
	}
	maxPoint := g.Columns[len(g.Columns)-1][len(g.Columns[0])-1]
	if newX > maxPoint.X {
		g.Columns = append(g.Columns, make([]*Point, len(g.Columns[0])))
		g.InitializeColumn(newX)
	}
	if newY > maxPoint.Y {
		for x := g.XOffset; x < len(g.Columns)+g.XOffset; x++ {
			index := x - g.XOffset
			g.Columns[index] = append(g.Columns[index], &Point{x, newY, false})
		}
	}

}

func (g Grid) GetPoint(x, y int) *Point {
	return g.Columns[x-g.XOffset][y-g.YOffset]
}

func (g *Grid) SetPoint(x, y int, p *Point) {
	g.Columns[x-g.XOffset][y-g.YOffset] = p
}

func (g *Grid) InitializeColumn(x int) {
	for y := g.YOffset; y < len(g.Columns[0])+g.YOffset; y++ {
		g.SetPoint(x, y, &Point{x, y, false})
	}
}

func (g *Grid) Move(command string) {
	direction := strings.Split(command, " ")[0]
	length, _ := strconv.Atoi(strings.Split(command, " ")[1])

	for i := 0; i < length; i++ {
		// Move Head to next position
		newX, newY := Step(g.Head.Position.X, g.Head.Position.Y, direction)
		if g.OutOfBounds(newX, newY) {
			g.ExtendGrid(newX, newY)
		}
		g.Head.Position = g.GetPoint(newX, newY)
		// Check relative knot positions
		current := g.Head
		for current.Next != nil {
			g.CascadeKnots(current, current.Next)
			current = current.Next
		}
		g.Tail.Position.TailVisited = true
	}
}

func (g *Grid) CascadeKnots(front, back *Knot) {
	diffX := front.Position.X - back.Position.X
	diffY := front.Position.Y - back.Position.Y
	if math.Abs(float64(diffX))+math.Abs(float64(diffY)) > 2 {
		newX, newY := 0, 0
		if diffX > 0 {
			newX = back.Position.X + 1
		} else {
			newX = back.Position.X - 1
		}
		if diffY > 0 {
			newY = back.Position.Y + 1
		} else {
			newY = back.Position.Y - 1
		}
		back.Position = g.GetPoint(newX, newY)
	} else if math.Abs(float64(diffX)) > 1 {
		back.Position = g.GetPoint(back.Position.X+diffX/2, back.Position.Y)
	} else if math.Abs(float64(diffY)) > 1 {
		back.Position = g.GetPoint(back.Position.X, back.Position.Y+diffY/2)
	}
}

func InitializeGrid(ropeLength int) Grid {

	initialSize := 11
	grid := Grid{Columns: make([][]*Point, initialSize)}
	grid.XOffset = -5
	grid.YOffset = -5
	for x := grid.XOffset; x < initialSize+grid.XOffset; x++ {
		grid.Columns[x-grid.XOffset] = make([]*Point, initialSize)
		grid.InitializeColumn(x)
	}
	grid.Start = grid.GetPoint(0, 0)
	grid.Head = &Knot{"H", grid.GetPoint(0, 0), nil}
	current := grid.Head
	for i := 1; i < ropeLength; i++ {
		current.Next = &Knot{fmt.Sprint(i), grid.GetPoint(0, 0), nil}
		current = current.Next
	}
	grid.Tail = current
	grid.Tail.Position.TailVisited = true

	return grid
}

func Step(x, y int, direction string) (int, int) {
	switch direction {
	case "R":
		return x + 1, y
	case "L":
		return x - 1, y
	case "U":
		return x, y + 1
	case "D":
		return x, y - 1
	}
	fmt.Println("Unknown Step Direction")
	return x, y
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
	grid := InitializeGrid(2)
	for _, line := range input {
		grid.Move(line)
	}

	return grid.TailVisited()
}

func calculateResultB(input []string) int {
	grid := InitializeGrid(10)
	for _, line := range input {
		grid.Move(line)
	}

	return grid.TailVisited()

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
