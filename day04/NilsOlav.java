import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class day4 {

    public static void main(String[] args) throws FileNotFoundException {

        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    private static int part1() throws FileNotFoundException {
        File myObj = new File("day4input");
        Scanner myReader = new Scanner(myObj);
        int sum = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            Range first = getFirstRange(line);
            Range second = getSecondRange(line);
            if (contained(first, second)) {
                sum++;
            }
        }
        return sum;
    }

    private static int part2() throws FileNotFoundException {
        File myObj = new File("day4input");
        Scanner myReader = new Scanner(myObj);
        int sum = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            Range first = getFirstRange(line);
            Range second = getSecondRange(line);
            if (overlap(first, second) || contained(first, second)) {
                sum++;
            }
        }
        return sum;
    }

    public static boolean overlap (Range r1, Range r2) {
        return (r1.low >= r2.low && r1.low <= r2.high) || (r1.high >= r2.low && r1.high <= r2.high);
    }
    public static boolean contained(Range r1, Range r2) {
        return (r1.low <= r2.low && r1.high >= r2.high) || (r1.low >= r2.low && r1.high <= r2.high);
    }

    public static Range getFirstRange(String line) {
        String range = line.split(",")[0];
        return getRange(range);
    }

    public static Range getSecondRange(String line) {
        String range = line.split(",")[1];
        return getRange(range);
    }

    public static Range getRange(String range) {
        String[] split = range.split("-");
        return new Range(split[0], split[1]);
    }

    public static class Range {
        public int low;
        public int high;

        public Range(String low, String high) {
            this.low = Integer.valueOf(low);
            this.high = Integer.valueOf(high);
        }
    }

}
