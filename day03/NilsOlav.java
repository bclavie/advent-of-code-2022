import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class day3 {
    public static void main(String[] args) throws FileNotFoundException {

        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    private static int part1() throws FileNotFoundException {
        File myObj = new File("day3input");
        Scanner myReader = new Scanner(myObj);
        int sum = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            Character matchingItem = findMatchingItem(line.substring(0, line.length() / 2), line.substring(line.length() / 2, line.length()));
            sum += getPriority(matchingItem);
        }
        return sum;
    }

    private static int part2() throws FileNotFoundException {
        File myObj = new File("day3input");
        Scanner myReader = new Scanner(myObj);
        int sum = 0;
        while (myReader.hasNextLine()) {
            String sack1 = myReader.nextLine();
            String sack2 = myReader.nextLine();
            String sack3 = myReader.nextLine();

            Character matchingItem = findMatchingItem(sack1, sack2, sack3);
            sum += getPriority(matchingItem);
        }
        return sum;
    }

    public static int getPriority(Character c) {
        int i = c - 96;
        if (i < 0) {
            return i + 58;
        } else {
            return i;
        }

    }

    public static Character findMatchingItem(String s1, String s2, String s3) {
        List<Character> l1 = stringToCharList(s1);
        List<Character> l2 = stringToCharList(s2);
        List<Character> l3 = stringToCharList(s3);

        for (Character c : l1) {
            if (l2.contains(c) && l3.contains(c)) {
                return c;
            }
        }

        throw new RuntimeException("No match");
    }

    public static Character findMatchingItem(String s1, String s2) {
        List<Character> l1 = stringToCharList(s1);
        List<Character> l2 = stringToCharList(s2);

        for (Character c : l1) {
            if (l2.contains(c)) {
                return c;
            }
        }

        throw new RuntimeException("No match");
    }

    public static List<Character> stringToCharList(String s) {
        char[] chars = s.toCharArray();
        List<Character> list = new ArrayList<>();
        for (char c : chars) {
            list.add(c);
        }
        return list;
    }
}
