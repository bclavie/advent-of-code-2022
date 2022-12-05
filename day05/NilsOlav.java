import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day5 {


    public static Map<Integer, String> getStacks() {
        return new HashMap<>(Map.of(1, "HRBDZFLS", 2, "TBMZR", 3, "ZLCHNS", 4, "SCFJ", 5, "PGHWRZB", 6, "VJZGDNMT", 7, "GLNWFSPQ", 8, "MZR", 9, "MCLGVRT"));
    }

    public static void main(String[] args) throws FileNotFoundException {

        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    private static String part1() throws FileNotFoundException {
        File myObj = new File("day5inputorders");
        Scanner myReader = new Scanner(myObj);
        Map<Integer, String> stacks = getStacks();
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            String[] move = line.split("move");
            String[] froms = move[1].split("from");
            String[] tos = froms[1].split("to");
            int amount = Integer.parseInt(froms[0].strip());
            int from = Integer.parseInt(tos[0].strip());
            int to = Integer.parseInt(tos[1].strip());
            moveCratesMultipleTimes(stacks, from, to, amount);
        }

        return getTopCrates(stacks);
    }

    private static String part2() throws FileNotFoundException {
        File myObj = new File("day5inputorders");
        Scanner myReader = new Scanner(myObj);
        Map<Integer, String> stacks = getStacks();
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            String[] move = line.split("move");
            String[] froms = move[1].split("from");
            String[] tos = froms[1].split("to");
            int amount = Integer.parseInt(froms[0].strip());
            int from = Integer.parseInt(tos[0].strip());
            int to = Integer.parseInt(tos[1].strip());
            moveMultipleCrates(stacks, from, to, amount);
        }

        return getTopCrates(stacks);
    }

    private static String getTopCrates(Map<Integer, String> stacks) {
        StringBuilder result = new StringBuilder();
        for (int i = 1; i <= stacks.size(); i++) {
            String s = stacks.get(i);
            result.append(s.substring(s.length() - 1));
        }
        return result.toString();
    }


    private static void moveMultipleCrates(Map<Integer, String> stacks, int from, int to, int amount) {
        String fromStack = stacks.get(from);
        String toStack = stacks.get(to);

        String crate = fromStack.substring(fromStack.length() - amount);
        stacks.put(from, fromStack.substring(0, fromStack.length() - amount));
        stacks.put(to, toStack + crate);

    }

    private static void moveCratesMultipleTimes(Map<Integer, String> stacks, int from, int to, int amount) {
        for (int i = 0; i < amount; i++) {
            moveCrate(stacks, from, to);
        }
    }

    private static void moveCrate(Map<Integer, String> stacks, int from, int to) {
        String fromStack = stacks.get(from);
        String toStack = stacks.get(to);

        String crate = fromStack.substring(fromStack.length() - 1);
        stacks.put(from, fromStack.substring(0, fromStack.length() - 1));
        stacks.put(to, toStack + crate);
    }
}
