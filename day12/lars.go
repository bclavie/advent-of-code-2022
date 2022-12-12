package main

import (
	"bufio"
	"fmt"
	"math"
	"os"

	"golang.org/x/exp/slices"
)

var file string

type Node struct {
	X         int
	Y         int
	Elevation string
	Edges     []*Node
	Distance  int
	Prev      *Node
}

func (n Node) String() string {
	return fmt.Sprintf("([%d][%d]:%s - Distance: %d) ", n.X, n.Y, n.Elevation, n.Distance)
}

func (n Node) Connects(other *Node) bool {
	return int(n.Elevation[0])-int(other.Elevation[0]) >= -1
}

type Graph struct {
	Visited   []*Node
	Unvisited []*Node
}

func (g Graph) String() string {
	text := "Visited: "
	for _, node := range g.Visited {
		text += node.String()
	}
	text += "\nUnvisited: "
	for _, node := range g.Unvisited {
		text += node.String()
	}
	text += "\n"

	return text
}

func (g *Graph) Next() (next *Node) {
	slices.SortFunc(g.Unvisited, func(a, b *Node) bool {
		return a.Distance <= b.Distance
	})
	next, g.Unvisited = g.Unvisited[0], g.Unvisited[1:]
	g.Visited = append(g.Visited, next)
	return next
}

type Grid struct {
	// [y][x] - top left 0/0
	Rows  [][]*Node
	Start *Node
	End   *Node
}

func (g Grid) String() string {
	text := ""
	for _, row := range g.Rows {
		for _, node := range row {
			if g.Start == node {
				text += "S"
			} else if g.End == node {
				text += "E"
			} else {
				text += node.Elevation
			}
		}
		text += "\n"
	}
	return text
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResult(input, false)
	}

	return calculateResult(input, true)
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

func GenerateGrid(input []string) Grid {
	grid := Grid{Rows: make([][]*Node, 0)}

	// Initialize Grid
	for y, line := range input {
		grid.Rows = append(grid.Rows, make([]*Node, len(line)))
		for x, rune := range line {
			char := string(rune)
			grid.Rows[y][x] = &Node{x, y, char, make([]*Node, 0), 0, nil}
			if char == "S" {
				grid.Rows[y][x].Elevation = "a"
				grid.Start = grid.Rows[y][x]
			}
			if char == "E" {
				grid.Rows[y][x].Elevation = "z"
				grid.End = grid.Rows[y][x]
			}
		}
	}

	return grid
}

func GenerateGraph(grid Grid, allAStart bool) *Graph {
	graph := Graph{make([]*Node, 0), make([]*Node, 0)}

	for y, row := range grid.Rows {
		for x, node := range row {
			// Set Dijkstra properties
			if node == grid.Start || (allAStart && node.Elevation == "a") {
				node.Distance = 0
			} else {
				node.Distance = math.MaxInt
			}
			graph.Unvisited = append(graph.Unvisited, node)
			// Create edges
			if y > 0 {
				top := grid.Rows[y-1][x]
				if node.Connects(top) {
					node.Edges = append(node.Edges, top)
				}
			}
			if x > 0 {
				left := grid.Rows[y][x-1]
				if node.Connects(left) {
					node.Edges = append(node.Edges, left)
				}
			}
			if y < len(grid.Rows)-1 {
				bottom := grid.Rows[y+1][x]
				if node.Connects(bottom) {
					node.Edges = append(node.Edges, bottom)
				}
			}
			if x < len(grid.Rows[y])-1 {
				right := grid.Rows[y][x+1]
				if node.Connects(right) {
					node.Edges = append(node.Edges, right)
				}
			}
		}
	}

	return &graph
}

func calculateResult(input []string, allAStart bool) int {

	grid := GenerateGrid(input)
	graph := GenerateGraph(grid, allAStart)

	current := graph.Next()
	for current != grid.End && current.Distance < math.MaxInt {
		for _, edge := range current.Edges {
			if current.Distance+1 < edge.Distance {
				edge.Distance = current.Distance + 1
				edge.Prev = current
			}
		}
		current = graph.Next()
	}

	return current.Distance
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
