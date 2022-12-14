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

type Material int

const (
	Air Material = iota
	Sand
	Stone
)

type Point struct {
	X int
	Y int
}

type Cave struct {
	// Columns[x][y] for access to coordinates
	Columns    [][]Material
	Depth      int
	SandSpawn  Point
	MovingSand Point
	SandCount  int
	BottomLess bool
}

func (c *Cave) Tick() (moved, done bool) {
	// Try down
	if c.GetPoint(c.MovingSand.X, c.MovingSand.Y+1) == Air {
		c.MovingSand.Y++
		return true, c.MovingSand.Y+1 > len(c.Columns[0])-1
	}
	// Try down left
	if c.GetPoint(c.MovingSand.X-1, c.MovingSand.Y+1) == Air {
		c.MovingSand.X--
		c.MovingSand.Y++
		if c.MovingSand.X <= 0 {
			c.AddColumn(true)
		}

		return true, c.MovingSand.Y+1 > len(c.Columns[0])-1
	}
	// Try down right
	if c.GetPoint(c.MovingSand.X+1, c.MovingSand.Y+1) == Air {
		c.MovingSand.X++
		c.MovingSand.Y++
		if c.MovingSand.X >= len(c.Columns)-1 {
			c.AddColumn(false)
		}
		return true, c.MovingSand.Y+1 > len(c.Columns[0])-1
	}

	return false, c.MovingSand == c.SandSpawn
}

func (c *Cave) ProduceSand(init bool) {
	if !init {
		c.SetPoint(c.MovingSand.X, c.MovingSand.Y, Sand)
	}
	c.MovingSand = Point{c.SandSpawn.X, c.SandSpawn.Y}
	c.SandCount++
}

func (c *Cave) AddColumn(prepend bool) {
	column := make([]Material, c.Depth)
	if !c.BottomLess {
		column[len(column)-1] = Stone
	}
	if prepend {
		c.Columns = append([][]Material{column}, c.Columns...)
		// Compensate position for sand
		c.SandSpawn.X++
		c.MovingSand.X++
	} else {
		c.Columns = append(c.Columns, column)
	}
}

func (c Cave) GetPoint(x, y int) Material {
	return c.Columns[x][y]
}

// Used in initialization with input thus using offset
func (c *Cave) SetPoint(x, y int, m Material) {
	c.Columns[x][y] = m
}

func (c Cave) String() string {
	text := ""
	for y := 0; y < len(c.Columns[0]); y++ {
		for x := 0; x < len(c.Columns); x++ {
			tile := c.GetPoint(x, y)
			if c.MovingSand.X == x && c.MovingSand.Y == y {
				text += "+"
			} else {
				switch tile {
				case Air:
					text += "."
				case Sand:
					text += "o"
				case Stone:
					text += "#"
				}
			}
		}
		text += "\n"
	}
	return text
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	return calculateResult(input, firstPart)
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

func ParseStonePoints(input []string) (stones [][]Point, minX, maxX, maxY int) {
	stones = make([][]Point, len(input))
	minX, maxX, maxY = 500, 500, 0
	for i, line := range input {
		pointStrings := strings.Split(line, " -> ")
		stones[i] = make([]Point, len(pointStrings))
		for j, point := range pointStrings {
			x, _ := strconv.Atoi(strings.Split(point, ",")[0])
			y, _ := strconv.Atoi(strings.Split(point, ",")[1])
			stones[i][j] = Point{x, y}
			if x < minX {
				minX = x
			}
			if x > maxX {
				maxX = x
			}
			if y > maxY {
				maxY = y
			}
		}
	}

	return
}

func InitializeCave(stones [][]Point, minX, maxX, maxY int, bottomLess bool) Cave {
	offset := minX - 1
	// Account for x offset for all spawning sand
	cave := Cave{SandSpawn: Point{500 - offset, 0}, BottomLess: bottomLess, Depth: maxY + 3}
	width := (maxX + 1) - (offset - 1)

	// Fill with air
	cave.Columns = make([][]Material, width)
	for x := 0; x < width; x++ {
		cave.Columns[x] = make([]Material, cave.Depth)
	}

	// Put stones
	for _, row := range stones {
		for i := 0; i < len(row)-1; i++ {
			// Account for x offset here to apply to whole grid
			from := Point{row[i].X - offset, row[i].Y}
			to := Point{row[i+1].X - offset, row[i+1].Y}
			xDiff := to.X - from.X
			if xDiff != 0 {
				xChange := (to.X - from.X) / int(math.Abs(float64(to.X-from.X)))
				for x := from.X; x != to.X; x += xChange {
					cave.SetPoint(x, from.Y, Stone)
				}
				cave.SetPoint(to.X, to.Y, Stone)
			}
			yDiff := to.Y - from.Y
			if yDiff != 0 {
				yChange := (to.Y - from.Y) / int(math.Abs(float64(to.Y-from.Y)))
				for y := from.Y; y != to.Y; y += yChange {
					cave.SetPoint(from.X, y, Stone)
				}
				cave.SetPoint(to.X, to.Y, Stone)
			}
		}
	}

	if !bottomLess {
		for x := 0; x < width; x++ {
			cave.SetPoint(x, cave.Depth-1, Stone)
		}
	}

	return cave
}

func calculateResult(input []string, firstPart bool) int {

	stones, minX, maxX, maxY := ParseStonePoints(input)
	cave := InitializeCave(stones, minX, maxX, maxY, firstPart)
	cave.ProduceSand(true)
	fmt.Println(cave)
	moved, done := false, false
	for !done {
		moved, done = cave.Tick()
		if !moved {
			cave.ProduceSand(false)
		}
	}
	fmt.Println(cave)

	return cave.SandCount - 1
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
