import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day6 {

    public static void main(String[] args) throws FileNotFoundException {

        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    private static int part1() throws FileNotFoundException {
        File myObj = new File("day6input");
        Scanner myReader = new Scanner(myObj);
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            char[] chars = line.toCharArray();
            List<Character> characters = new ArrayList<>();
            for (int i = 0; i < chars.length; i++) {
                Character c = chars[i];
                characters.add(c);
                if (characters.size() == 4) {
                    if (noDuplicatesInList(characters)) {
                        return i + 1;
                    }
                    characters.remove(0);
                }
            }
        }

        throw new RuntimeException("No sequence found");
    }

    private static int part2() throws FileNotFoundException {
        File myObj = new File("day6input");
        Scanner myReader = new Scanner(myObj);
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            char[] chars = line.toCharArray();
            List<Character> characters = new ArrayList<>();
            for (int i = 0; i < chars.length; i++) {
                Character c = chars[i];
                characters.add(c);
                if (characters.size() == 14) {
                    if (noDuplicatesInList(characters)) {
                        return i + 1;
                    }
                    characters.remove(0);
                }
            }
        }

        throw new RuntimeException("No sequence found");
    }

    public static boolean noDuplicatesInList(List<Character> list) {
        for (Character c : list) {
            if (list.indexOf(c) != list.lastIndexOf(c)) {
                return false;
            }
        }
        return true;
    }


}
