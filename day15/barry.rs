use aoc2022::time_run2;
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../inputs/15");

#[time_run2("15")]
fn main() {
    beacon_exclusion(INPUT)
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq)]
struct Point(i64, i64);

fn manhattan_distance(a: Point, b: Point) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Sensor {
    location: Point,
    closest_beacon: Point,
    man_distance: i64,
}

impl Sensor {
    fn new(location: Point, closest_beacon: Point) -> Self {
        let man_distance = manhattan_distance(location, closest_beacon);
        Self {
            location,
            closest_beacon,
            man_distance,
        }
    }

    fn find_points_in_row_y_within_range(&self, y: i64) -> Vec<Point> {
        let mut points = vec![];

        let y_diff = manhattan_distance(self.location, Point(self.location.0, y));
        let y_range = self.man_distance - y_diff;
        // The y coord to check is larger than the sensor's coverage range.
        if y_range < 0 {
            return points;
        }

        for x in (self.location.0 - y_range)..=(self.location.0 + y_range) {
            points.push(Point(x, y))
        }

        points
    }

    // Returns whether or not the point is in range
    fn is_point_in_detection_range(&self, p: Point) -> bool {
        manhattan_distance(self.location, p) <= self.man_distance
    }
}

fn find_distress_signal_point(sensors: Vec<Sensor>) -> Point {
    // Just need to find a single point in the range
    // which is greater than all the manhattan distances of the sensors
    // and their corresponding beacons.
    // Instead of checking x 0..=4000000 y 0..=4000000
    // We can check a reduced subset by checking each coordinate
    // just outside of each sensor range.
    let mut points_to_check: Vec<Point> = vec![];
    for s in &sensors {
        for x_diff in 0..=s.man_distance + 1 {
            let y_diff = s.man_distance + 1 - x_diff;

            let x = s.location.0 - x_diff;
            let y = s.location.1 - y_diff;

            if (0..=4000000).contains(&x) && (0..=4000000).contains(&y) {
                points_to_check.push(Point(x, y))
            }

            let x = s.location.0 + x_diff;
            let y = s.location.1 - y_diff;

            if (0..=4000000).contains(&x) && (0..=4000000).contains(&y) {
                points_to_check.push(Point(x, y))
            }

            let x = s.location.0 - x_diff;
            let y = s.location.1 + y_diff;

            if (0..=4000000).contains(&x) && (0..=4000000).contains(&y) {
                points_to_check.push(Point(x, y))
            }

            let x = s.location.0 + x_diff;
            let y = s.location.1 + y_diff;

            if (0..=4000000).contains(&x) && (0..=4000000).contains(&y) {
                points_to_check.push(Point(x, y))
            }
        }
    }

    for point in points_to_check {
        let mut is_point_distress_beacon = true;
        for s in &sensors {
            if s.is_point_in_detection_range(point) {
                is_point_distress_beacon = false;
                break;
            }
        }

        if is_point_distress_beacon {
            return point;
        }
    }

    panic!("failed to find")
}

fn beacon_exclusion(i: &str) -> (String, String) {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let sensors: Vec<Sensor> = i
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Sensor::new(
                Point(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                Point(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            )
        })
        .collect();

    let points = sensors
        .iter()
        .flat_map(|sensor| sensor.find_points_in_row_y_within_range(2000000))
        .unique();

    // Need to remove all the coords which contain an occupied space
    let taken_points: Vec<Point> = sensors
        .iter()
        .flat_map(|sensor| vec![sensor.closest_beacon, sensor.location])
        .collect();

    let part_1 = points
        .into_iter()
        .filter(|point| !taken_points.contains(point))
        .count();

    let distress_beacon_point = find_distress_signal_point(sensors);
    dbg!(distress_beacon_point);
    let part2 = (4000000 * distress_beacon_point.0) + distress_beacon_point.1;

    (part_1.to_string(), part2.to_string())
}
