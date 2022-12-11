import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day9 {

    public static void main(String[] args) throws FileNotFoundException {
        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    public static int part1() throws FileNotFoundException {
        List<Coordinate> rope = List.of(new Coordinate(0, 0), new Coordinate(0, 0));
        return moveRope(rope);
    }

    public static int part2() throws FileNotFoundException {
        List<Coordinate> rope = List.of(new Coordinate(0, 0),
                new Coordinate(0, 0), new Coordinate(0, 0), new Coordinate(0, 0),
                new Coordinate(0, 0), new Coordinate(0, 0), new Coordinate(0, 0),
                new Coordinate(0, 0), new Coordinate(0, 0), new Coordinate(0, 0));
        return moveRope(rope);
    }

    public static int moveRope(List<Coordinate> rope) throws FileNotFoundException {
        File myObj = new File("day9input");
        Scanner myReader = new Scanner(myObj);
        Set<Coordinate> visitedLocations = new HashSet<>();

        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            String[] move = line.split(" ");
            String dir = move[0];
            int times = Integer.parseInt(move[1]);

            for (int i = 0; i < times; i++) {
                if (dir.equals("R")) {
                    moveRight(rope);
                }

                if (dir.equals("L")) {
                    moveLeft(rope);
                }

                if (dir.equals("U")) {
                    moveUp(rope);
                }

                if (dir.equals("D")) {
                    moveDown(rope);
                }
                Coordinate tail = rope.get(rope.size() - 1);
                visitedLocations.add(new Coordinate(tail.x, tail.y));
            }


        }
        return visitedLocations.size();
    }

    public static void moveRight(List<Coordinate> rope) {
        rope.get(0).x++;
        for (int i = 1; i < rope.size(); i++) {
            Coordinate head = rope.get(i - 1);
            Coordinate tail = rope.get(i);
            adjustTail(head, tail);
        }
    }

    public static void moveLeft(List<Coordinate> rope) {
        rope.get(0).x--;
        for (int i = 1; i < rope.size(); i++) {
            Coordinate head = rope.get(i - 1);
            Coordinate tail = rope.get(i);
            adjustTail(head, tail);
        }
    }

    public static void moveUp(List<Coordinate> rope) {
        rope.get(0).y++;
        for (int i = 1; i < rope.size(); i++) {
            Coordinate head = rope.get(i - 1);
            Coordinate tail = rope.get(i);
            adjustTail(head, tail);
        }
    }

    public static void moveDown(List<Coordinate> rope) {
        rope.get(0).y--;
        for (int i = 1; i < rope.size(); i++) {
            Coordinate head = rope.get(i - 1);
            Coordinate tail = rope.get(i);
            adjustTail(head, tail);
        }
    }
    public static void adjustTail(Coordinate head, Coordinate tail) {
        if (tail.x + 2 == head.x) {
            if (tail.y < head.y) {
                tail.y++;
            } else if (tail.y > head.y) {
                tail.y--;
            }
            tail.x = head.x - 1;
        }

        if (tail.x - 2 == head.x) {
            if (tail.y < head.y) {
                tail.y++;
            } else if (tail.y > head.y) {
                tail.y--;
            }
            tail.x = head.x + 1;
        }

        if (tail.y + 2 == head.y) {
            if (tail.x < head.x) {
                tail.x++;
            } else if (tail.x > head.x) {
                tail.x--;
            }
            tail.y = head.y - 1;
        }

        if (tail.y - 2 == head.y) {
            if (tail.x < head.x) {
                tail.x++;
            } else if (tail.x > head.x) {
                tail.x--;
            }
            tail.y = head.y + 1;
        }
    }

    private static class Coordinate {
        int x;
        int y;

        public Coordinate(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Coordinate that = (Coordinate) o;
            return x == that.x && y == that.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }

        @Override
        public String toString() {
            return "{" +
                    "x=" + x +
                    ", y=" + y +
                    '}';
        }
    }
}
