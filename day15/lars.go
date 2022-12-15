package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"

	"golang.org/x/exp/slices"
)

var row int
var dimension int
var file string

var (
	fullRE = regexp.MustCompile(`Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)`)
)

type Point struct {
	X int
	Y int
}

func (p Point) String() string {
	return fmt.Sprintf("%d/%d", p.X, p.Y)
}

type Sensor struct {
	Point
	Beacon Point
}

func (s Sensor) String() string {
	return fmt.Sprintf("(%s):%d", s.Point, s.Range())
}

func (s Sensor) Range() int {
	return Distance(s.Point, s.Beacon)
}

func (s Sensor) InRange(p Point) bool {
	sensorRange := Distance(s.Point, s.Beacon)
	return Distance(s.Point, p) <= sensorRange
}

func Distance(a, b Point) int {
	return int(math.Abs(float64(a.X)-float64(b.X)) + math.Abs(float64(a.Y)-float64(b.Y)))
}

// HasPointsBetweenSensors Checks if the point can be between those two sensors
func HasPointsBetweenSensors(a, b Sensor) bool {
	// Since there is only one point that could be it, there has to be a gap between sensor coverage of exactly one point
	// And then it intersects with at least one other gap between sensors, which is where the point will be
	// Therefore the difference between the distance and the sum of the ranges of the sensors has to be 2
	if Distance(a.Point, b.Point)-(a.Range()+b.Range()) == 2 {
		return true
	}
	return false
}

// PointsBetweenSensors Gives all possible points from the gap line between sensors
func PointsBetweenSensors(from, to Sensor) []Point {
	possible := []Point{}
	xDiff := to.X - from.X
	xChange := (xDiff) / int(math.Abs(float64(xDiff))) // + 1 or -1
	yDiff := to.Y - from.Y
	yChange := (yDiff) / int(math.Abs(float64(yDiff))) // + 1 or -1
	// first we get the orientation of the sensors and then we walk along the gap to check which fulfill the proper ranges
	for i := 0; i <= from.Range(); i++ {
		test := Point{from.X + i*xChange + 1, from.Y + (from.Range()-i)*yChange}
		if Distance(test, from.Point) == from.Range()+1 && Distance(test, to.Point) == to.Range()+1 {
			possible = append(possible, test)
		}
	}

	return possible
}

type Itemtype int

const (
	Nothing Itemtype = iota
	SensorType
	BeaconType
)

func getResult(part string) int {
	input := getInput()
	firstPart := part == "A"

	sensors := ParseSensors(input)

	if firstPart {
		if row == 0 {
			row = 2000000
		}
		return calculateResultA(sensors, row)
	}

	if dimension == 0 {
		dimension = 4000000
	}

	return calculateResultB(sensors, dimension)
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

func ParseSensors(input []string) (sensors []Sensor) {
	for _, line := range input {
		matches := fullRE.FindStringSubmatch(line)
		sensorX, _ := strconv.Atoi(matches[1])
		sensorY, _ := strconv.Atoi(matches[2])
		beaconX, _ := strconv.Atoi(matches[3])
		beaconY, _ := strconv.Atoi(matches[4])
		sensors = append(sensors, Sensor{Point{sensorX, sensorY}, Point{beaconX, beaconY}})
	}

	return
}

func calculateResultA(sensors []Sensor, y int) int {

	relevantSensors := []Sensor{}
	// Filter for sensors that have the row y in range
	for _, sensor := range sensors {
		if Distance(sensor.Point, Point{sensor.X, y}) <= sensor.Range() {
			relevantSensors = append(relevantSensors, sensor)
		}
	}
	// For all sensors in range add their x values into the map
	points := make(map[int]Itemtype)
	for _, sensor := range relevantSensors {
		for x := sensor.X - sensor.Range(); x < sensor.X+sensor.Range(); x++ {
			if sensor.InRange(Point{x, y}) {
				points[x] = Nothing
			}
		}
	}
	// Now overwrite all values in the map with sensors and beacons in case they lie in there
	for _, sensor := range sensors {
		if sensor.Y == y {
			points[sensor.X] = SensorType
		}
		if sensor.Beacon.Y == y {
			points[sensor.Beacon.X] = BeaconType
		}
	}
	result := 0
	// Count all points in the map where there is nothing
	for _, value := range points {
		if value == Nothing {
			result++
		}
	}

	return result
}

func calculateResultB(sensors []Sensor, dimension int) int {

	slices.SortFunc(sensors, func(a, b Sensor) bool {
		aSum := a.X + a.Y
		bSum := b.X + b.Y
		if aSum == bSum {
			return a.Range() < b.Range()
		}
		return aSum < bSum
	})

	for a, outer := range sensors {
		for b, inner := range sensors {
			if b > a && HasPointsBetweenSensors(outer, inner) {
				// Get all possible points in the gaps between sensor coverage
				possiblePoints := PointsBetweenSensors(outer, inner)
				// then test if any other sensor (not from the pair) covers it
				for _, point := range possiblePoints {
					sensorFound := false
					for _, test := range sensors {
						if test.InRange(point) {
							sensorFound = true
							break
						}
					}
					if !sensorFound {
						return point.X*4000000 + point.Y
					}
				}
			}
		}
	}

	result := -1

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
