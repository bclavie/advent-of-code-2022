import java.io.File;
import java.io.FileNotFoundException;
import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;
import java.util.concurrent.atomic.AtomicInteger;

public class day2 {
    /**
     * A = Rock
     * B = Paper
     * C = Scissors
     * X = Rock, 1pts
     * Y = Paper, 2 pts
     * Z = Scissors, 3 pts
     * <p>
     * Loss = 0 pts
     * Draw = 3 pts
     * Win = 6 pts
     */
    public static Map<String, Integer> pointsMap = new HashMap<>();

    public static void main(String[] args) {
        pointsMap.put("WA", 2);
        pointsMap.put("WB", 3);
        pointsMap.put("WC", 1);

        pointsMap.put("LA", 3);
        pointsMap.put("LB", 1);
        pointsMap.put("LC", 2);

        pointsMap.put("A", 1);
        pointsMap.put("B", 2);
        pointsMap.put("C", 3);
        try {
            File myObj = new File("day2input");
            Scanner myReader = new Scanner(myObj);
            int score = 0;
            int part2 = 0;
            while (myReader.hasNextLine()) {
                String line = myReader.nextLine();
                score += calculatePointFromSelection(line);
                score += calculatePointsFromResult(line);
                part2 += calculatePart2(line);
            }
            System.out.println(score);
            System.out.println(part2);
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
    }
    public static int calculatePart2(String s) {
        String[] split = s.split(" ");
        String elf = split[0];
        String player = split[1];
        if(player.equals("X")) {
            return pointsMap.get("L" + elf);
        } else if(player.equals("Y")) {
            return pointsMap.get(elf) + 3;
        } else {
            return pointsMap.get("W" + elf) + 6;
        }
    }

    public static int calculatePointFromSelection(String s) {
        if (s.contains("X")) {
            return 1;
        } else if (s.contains("Y")) {
            return 2;
        } else {
            return 3;
        }
    }

    public static int calculatePointsFromResult(String s) {
        String[] split = s.split(" ");
        String elf = split[0];
        String player = split[1];

        if (elf.equals("A")) {
            switch (player) {
                case "X":
                    return 3;
                case "Y":
                    return 6;
                case "Z":
                    return 0;
            }
        } else if (elf.equals("B")) {
            switch (player) {
                case "X":
                    return 0;
                case "Y":
                    return 3;
                case "Z":
                    return 6;
            }
        } else if (elf.equals("C")) {
            switch (player) {
                case "X":
                    return 6;
                case "Y":
                    return 0;
                case "Z":
                    return 3;
            }
        }

        throw new RuntimeException("Non matching result");
    }
}
