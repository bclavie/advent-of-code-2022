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
	cdRE   = regexp.MustCompile(`\$ cd (.+)`)
	lsRE   = regexp.MustCompile(`\$ ls`)
	dirRE  = regexp.MustCompile(`dir (.+)`)
	fileRE = regexp.MustCompile(`(\d+) (.+)`)
)

type Filetype string

const (
	Directory Filetype = "directory"
	File      Filetype = "file"
)

type Path struct {
	Parent   *Path
	Name     string
	FullName string
	Type     Filetype
	Children []*Path
	size     int
}

func (p Path) Size() int {
	sum := p.size
	for _, child := range p.Children {
		sum += child.Size()
	}
	return sum
}

func (p Path) FileSizeSum() int {
	sum := 0
	for _, child := range p.Children {
		if child.Type == File {
			sum += child.size
		} else {
			sum += child.FileSizeSum()
		}
	}
	return sum
}

func (p Path) SizeMap() map[string]int {
	sizeMap := make(map[string]int)
	sizeMap[p.FullName] = p.Size()
	for _, child := range p.Children {
		if child.Type == Directory {
			for i, el := range child.SizeMap() {
				sizeMap[i] = el
			}
		}
	}
	return sizeMap
}

func (p Path) Depth() int {
	depth := 0
	position := &p
	for position.Name != "/" {
		position = position.Parent
		depth++
	}
	return depth
}

func (p Path) HasChild(name string) *Path {
	for _, child := range p.Children {
		if child.Name == name {
			return child
		}
	}
	return nil
}

func (p Path) ChangeDirectory(name string) *Path {
	if child := p.HasChild(name); child != nil {
		return child
	}
	return nil
}

func (p *Path) AddDirectory(name string) {
	newChild := &Path{Name: name, Type: Directory, Parent: p, FullName: fmt.Sprintf("%s%s/", p.FullName, name)}
	p.Children = append(p.Children, newChild)
}

func (p *Path) AddFile(name string, size int) {
	if child := p.HasChild(name); child == nil {
		newChild := &Path{Name: name, Type: File, Parent: p, size: size, FullName: fmt.Sprintf("%s%s", p.FullName, name)}
		p.Children = append(p.Children, newChild)
	}
}

func (p Path) String() string {
	text := fmt.Sprintf("%s[%s]", strings.Repeat("-", p.Depth()), p.Name)
	switch p.Type {
	case Directory:
		text += "\n"
		for _, child := range p.Children {
			text += child.String()
		}
	case File:
		text += fmt.Sprintf(" - %v \n", p.size)
	}
	return text
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

	fileSum := 0
	for scanner.Scan() {
		line := scanner.Text()
		if fileRE.MatchString(line) {
			matches := fileRE.FindStringSubmatch(line)
			size, _ := strconv.Atoi(matches[1])
			fileSum += size
		}
		input = append(input, line)
	}

	return input
}

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	if firstPart {
		return calculateResultA(input)
	}

	return calculateResultB(input)
}

func ConstructFilesystem(input []string) *Path {
	var root *Path
	var position *Path
	fileSum := 0
	i := 0
	for i < len(input) {
		line := input[i]
		if cdRE.MatchString(line) {
			matches := cdRE.FindStringSubmatch(line)
			dir := matches[1]
			switch dir {
			case "/":
				if root == nil {
					root = &Path{Name: dir, Type: Directory, FullName: dir}
				}
				position = root
			case "..":
				position = position.Parent
			default:
				position = position.ChangeDirectory(dir)
			}
			i++
		} else if lsRE.MatchString(line) {
			i++
			for i < len(input) && !strings.HasPrefix(input[i], "$") {
				line := input[i]
				if dirRE.MatchString(line) {
					matches := dirRE.FindStringSubmatch(line)
					dir := matches[1]
					position.AddDirectory(dir)
				} else if fileRE.MatchString(line) {
					matches := fileRE.FindStringSubmatch(line)
					size, _ := strconv.Atoi(matches[1])
					name := matches[2]
					fileSum += size
					position.AddFile(name, size)
				}
				i++
			}
		}
	}

	fmt.Println(root)

	return root
}

func calculateResultA(input []string) int {
	result := 0
	root := ConstructFilesystem(input)
	dirSizes := root.SizeMap()
	for _, size := range dirSizes {
		if size <= 100000 {
			result += size
		}
	}

	return result
}

func calculateResultB(input []string) int {
	totalSpace := 70000000
	neededSpace := 30000000
	root := ConstructFilesystem(input)
	unusedSpace := totalSpace - root.Size()
	toBeFreedUp := neededSpace - unusedSpace
	dirSizes := root.SizeMap()
	options := []int{}
	for _, size := range dirSizes {
		if size >= toBeFreedUp {
			options = append(options, size)
		}
	}

	slices.Sort(options)

	return options[0]

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
