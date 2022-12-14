import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class day14 {

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part1() throws FileNotFoundException {
        int lowestX = Integer.MAX_VALUE;
        int highestX = Integer.MIN_VALUE;
        int numberOfLines = Integer.MIN_VALUE;
        char[][] arena = new char[1000][1000];
        for (int i = 0; i < arena.length; i++) {
            for (int j = 0; j < arena[i].length; j++) {
                arena[i][j] = '.';
            }
        }
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day14input");
        //File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day14inputsmall");
        Scanner myReader = new Scanner(myObj);
        while (myReader.hasNext()) {
            String line = myReader.next();
            String[] split = line.split("->");
            for (int i = 0; i < split.length - 1; i++) {
                Point p1 = new Point(split[i]);
                Point p2 = new Point(split[i + 1]);

                if (p1.x < lowestX) {
                    lowestX = p1.x;
                }
                if (p2.x < lowestX) {
                    lowestX = p2.x;
                }

                if (p1.x > highestX) {
                    highestX = p1.x;
                }
                if (p2.x > highestX) {
                    highestX = p2.x;
                }

                if (p1.y > numberOfLines) {
                    numberOfLines = p1.y;
                }
                if (p2.y > numberOfLines) {
                    numberOfLines = p2.y;
                }

                if (p1.x != p2.x) {
                    if (p1.x > p2.x) {
                        drawHorizontal(arena, p2.x, p1.x, p1.y);
                    } else {
                        drawHorizontal(arena, p1.x, p2.x, p1.y);
                    }
                } else {

                    if (p1.y > p2.y) {
                        drawVertical(arena, p2.y, p1.y, p1.x);
                    } else {
                        drawVertical(arena, p1.y, p2.y, p1.x);
                    }
                }
            }
        }
        int sandCount = 0;
        while (moveSand(arena, new Point(500, 0), numberOfLines)) {
            sandCount++;
        }

        System.out.println("sand p1: " + sandCount);
    }

    public static void part2() throws FileNotFoundException {
        int lowestX = Integer.MAX_VALUE;
        int highestX = Integer.MIN_VALUE;
        int numberOfLines = Integer.MIN_VALUE;
        char[][] arena = new char[1000][1000];
        for (int i = 0; i < arena.length; i++) {
            for (int j = 0; j < arena[i].length; j++) {
                arena[i][j] = '.';
            }
        }
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day14input");
        //File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day14inputsmall");
        Scanner myReader = new Scanner(myObj);
        while (myReader.hasNext()) {
            String line = myReader.next();
            String[] split = line.split("->");
            for (int i = 0; i < split.length - 1; i++) {
                Point p1 = new Point(split[i]);
                Point p2 = new Point(split[i + 1]);

                if (p1.x < lowestX) {
                    lowestX = p1.x;
                }
                if (p2.x < lowestX) {
                    lowestX = p2.x;
                }

                if (p1.x > highestX) {
                    highestX = p1.x;
                }
                if (p2.x > highestX) {
                    highestX = p2.x;
                }

                if (p1.y > numberOfLines) {
                    numberOfLines = p1.y;
                }
                if (p2.y > numberOfLines) {
                    numberOfLines = p2.y;
                }

                if (p1.x != p2.x) {
                    if (p1.x > p2.x) {
                        drawHorizontal(arena, p2.x, p1.x, p1.y);
                    } else {
                        drawHorizontal(arena, p1.x, p2.x, p1.y);
                    }
                } else {
                    if (p1.y > p2.y) {
                        drawVertical(arena, p2.y, p1.y, p1.x);
                    } else {
                        drawVertical(arena, p1.y, p2.y, p1.x);
                    }
                }
            }
        }

        drawHorizontal(arena, 0, 999, numberOfLines + 2);

        int sandCount = 0;
        while (arena[500][0] != 'o') {
            moveSand2(arena, new Point(500, 0));
            sandCount++;
            //printArena(arena, lowestX, highestX, numberOfLines);
        }
        //printArena(arena, lowestX, highestX, numberOfLines);

        System.out.println("sand p2: " + sandCount);
    }

    private static boolean moveSand(char[][] arena, Point sand, int numberOfLines) {
        char pointBelow = arena[sand.x][sand.y + 1];
        char pointLeft = arena[sand.x - 1][sand.y + 1];
        char pointRight = arena[sand.x + 1][sand.y + 1];
        if (sand.y > numberOfLines) {
            return false;
        } else if (pointBelow == '.') {
            return moveSand(arena, new Point(sand.x, sand.y + 1), numberOfLines);
        } else if (pointLeft == '.') {
            return moveSand(arena, new Point(sand.x - 1, sand.y + 1), numberOfLines);
        } else if (pointRight == '.') {
            return moveSand(arena, new Point(sand.x + 1, sand.y + 1), numberOfLines);
        } else {
            arena[sand.x][sand.y] = 'o';
            return true;
        }
    }

    private static void moveSand2(char[][] arena, Point sand) {
        char pointBelow = arena[sand.x][sand.y + 1];
        char pointLeft = arena[sand.x - 1][sand.y + 1];
        char pointRight = arena[sand.x + 1][sand.y + 1];
        if (pointBelow == '.') {
            moveSand2(arena, new Point(sand.x, sand.y + 1));
        } else if (pointLeft == '.') {
            moveSand2(arena, new Point(sand.x - 1, sand.y + 1));
        } else if (pointRight == '.') {
            moveSand2(arena, new Point(sand.x + 1, sand.y + 1));
        } else {
            arena[sand.x][sand.y] = 'o';
        }
    }


    private static void printArena(char[][] arena, int lowestX, int highestX, int numberOfLines) {
        System.out.println("lowestX: " + lowestX);
        System.out.println("highestX: " + highestX);
        System.out.println("numberOfLines: " + numberOfLines);
        for (int j = 0; j <= numberOfLines; j++) {
            String line = "";
            for (int i = lowestX; i <= highestX; i++) {
                line += arena[i][j];
            }
            System.out.println(line);
        }
    }

    //Assume x1 >= x2
    public static void drawHorizontal(char[][] arena, int x1, int x2, int y) {
        for (int i = x1; i <= x2; i++) {
            arena[i][y] = '#';
        }
    }

    public static void drawVertical(char[][] arena, int y1, int y2, int x) {
        for (int i = y1; i <= y2; i++) {
            arena[x][i] = '#';
        }
    }

    public static class Point {
        public int x;
        public int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        public Point(String s) {
            String[] split = s.split(",");
            x = Integer.parseInt(split[0]);
            y = Integer.parseInt(split[1]);
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
