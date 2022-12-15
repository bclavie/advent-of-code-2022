import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day15 {
    public static void main(String[] args) throws FileNotFoundException {
        long start = System.currentTimeMillis();
        part1();
        long part1 = System.currentTimeMillis();
        part2();
        long part2 = System.currentTimeMillis();

        System.out.println("Part 1 time (ms): " + (part1 - start));
        System.out.println("Part 2 time (ms): " + (part2 - part1));
    }

    public static void part2() throws FileNotFoundException {
        //File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day15inputsmall");
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day15input");
        Scanner myReader = new Scanner(myObj);
        Map<Point, Point> sensorBeacon = new HashMap<>();
        Map<Point, Integer> sensorDelta = new HashMap<>();
        int smallestX = Integer.MAX_VALUE;
        int largestX = Integer.MIN_VALUE;
        int largestDelta = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            String[] split = line.split(": closest beacon is at ");
            Point sensor = new Point(split[0].split(" at ")[1]);
            Point beacon = new Point(split[1]);
            if (sensor.x < smallestX) {
                smallestX = sensor.x;
            }
            if (sensor.x > largestX) {
                largestX = sensor.x;
            }
            sensorBeacon.put(sensor, beacon);
            int delta = taxiDistance(sensor, beacon);
            if (delta > largestDelta) {
                largestDelta = delta;
            }
            sensorDelta.put(sensor, delta);
        }
        Set<Point> points = new HashSet<>();

        addPoints(sensorDelta, points);

        findPoint(sensorDelta, points);
    }

    public static void findPoint(Map<Point, Integer> sensorDeltas, Set<Point> points) {
        for (Point test : points) {
            boolean found = true;
            for (Map.Entry<Point, Integer> sensorDelta : sensorDeltas.entrySet()) {
                if (taxiDistance(test, sensorDelta.getKey()) <= sensorDelta.getValue()) {
                    found = false;
                    break;
                }
            }
            if (found) {
                System.out.println("found: " + test);
                System.out.println(4000000L * test.x + test.y);
                break;
            }
        }
    }

    public static void addPoints(Map<Point, Integer> sensorDeltas, Set<Point> points) {
        for (Map.Entry<Point, Integer> sensorDelta : sensorDeltas.entrySet()) {
            addPoints(sensorDelta.getKey(), sensorDelta.getValue(), points);
        }
    }

    public static void addPoints(Point point, int delta, Set<Point> points) {
        int x = point.x;
        int y = point.y;
        int offset = delta + 1;
        int upperBound = 4000000;

        for (int i = 0; i <= offset; i++) {
            int xRight = x + (offset - i);
            int xLeft = x - (offset - i);
            int yUp = y - i;
            int yDown = y + i;
            if (xRight > 0 && xRight <= upperBound) {
                if (yUp > 0 && yUp <= upperBound) {
                    points.add(new Point(xRight, yUp));
                }
                if (yDown > 0 && yDown <= upperBound) {
                    points.add(new Point(xRight, yDown));
                }
            }

            if (xLeft > 0 && xLeft <= upperBound) {
                if (yUp > 0 && yUp <= upperBound) {
                    points.add(new Point(xLeft, yUp));
                }
                if (yDown > 0 && yDown <= upperBound) {
                    points.add(new Point(xLeft, yDown));
                }
            }
        }
    }

    public static void part1() throws FileNotFoundException {
        //File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day15inputsmall");
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day15input");
        Scanner myReader = new Scanner(myObj);
        Map<Point, Point> sensorBeacon = new HashMap<>();
        Map<Point, Integer> sensorDelta = new HashMap<>();
        int smallestX = Integer.MAX_VALUE;
        int largestX = Integer.MIN_VALUE;
        int largestDelta = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            String[] split = line.split(": closest beacon is at ");
            Point sensor = new Point(split[0].split(" at ")[1]);
            Point beacon = new Point(split[1]);
            if (sensor.x < smallestX) {
                smallestX = sensor.x;
            }
            if (sensor.x > largestX) {
                largestX = sensor.x;
            }
            sensorBeacon.put(sensor, beacon);
            int delta = taxiDistance(sensor, beacon);
            if (delta > largestDelta) {
                largestDelta = delta;
            }
            sensorDelta.put(sensor, delta);
        }

        int cannotBePresent = 0;
        for (int i = smallestX - largestDelta; i < largestX + largestDelta; i++) {
            Point test = new Point(i, 2000000);
            if (!sensorBeacon.containsValue(test)) {
                for (Point sensor : sensorDelta.keySet()) {
                    if (taxiDistance(test, sensor) <= sensorDelta.get(sensor)) {
                        cannotBePresent++;
                        break;
                    }
                }
            }
        }
        System.out.println(cannotBePresent);
    }

    public static int taxiDistance(Point p1, Point p2) {
        return delta(p1.x, p2.x) + delta(p1.y, p2.y);
    }

    public static int delta(int a, int b) {
        if ((a >= 0 && b >= 0) || (a <= 0 && b <= 0)) {
            return Math.abs(Math.abs(a) - Math.abs(b));
        } else if (a >= 0) {
            return a + Math.abs(b);
        } else {
            return Math.abs(a) + b;
        }
    }

    public static class Point {
        public int x;
        public int y;

        //x=16, y=7
        public Point(String s) {
            String[] split = s.split(", y=");
            this.x = Integer.parseInt(split[0].split("x=")[1]);
            this.y = Integer.parseInt(split[1]);

        }

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return x == point.x && y == point.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }

        @Override
        public String toString() {
            return "Point{" +
                    "x=" + x +
                    ", y=" + y +
                    '}';
        }
    }
}
