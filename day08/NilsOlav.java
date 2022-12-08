import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class day8 {

    public static void main(String[] args) throws FileNotFoundException {
        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    public static int part1() throws FileNotFoundException {
        return countVisibleTrees(getTrees());
    }

    public static int part2() throws FileNotFoundException {
        return getHighestScenicScore(getTrees());
    }

    public static int[][] getTrees() throws FileNotFoundException {
        File myObj = new File("day8input");
        Scanner myReader = new Scanner(myObj);
        int[][] trees = new int[99][99];
        int rowIndex = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            char[] chars = line.toCharArray();
            for (int i = 0; i < chars.length; i++) {
                trees[rowIndex][i] = Integer.parseInt(String.valueOf(chars[i]));
            }
            rowIndex++;
        }
        return trees;
    }

    public static int countVisibleTrees(int[][] trees) {
        int visibleTrees = 0;
        for (int x = 0; x < 99; x++) {
            for (int y = 0; y < 99; y++) {
                if (checkTreeIsVisible(trees, x, y)) {
                    visibleTrees++;
                }
            }
        }

        return visibleTrees;
    }

    public static boolean checkTreeIsVisible(int[][] trees, int x, int y) {
        return checkWest(trees, x, y) || checkEast(trees, x, y)
                || checkNorth(trees, x, y) || checkSouth(trees, x, y);
    }

    public static boolean checkWest(int[][] trees, int x, int y) {
        int compareTree = trees[x][y];
        for (int i = x - 1; i >= 0; i--) {
            if (trees[i][y] >= compareTree) {
                return false;
            }
        }
        return true;
    }

    public static boolean checkEast(int[][] trees, int x, int y) {
        int compareTree = trees[x][y];
        for (int i = x + 1; i < 99; i++) {
            if (trees[i][y] >= compareTree) {
                return false;
            }
        }
        return true;
    }

    public static boolean checkNorth(int[][] trees, int x, int y) {
        int compareTree = trees[x][y];
        for (int i = y - 1; i >= 0; i--) {
            if (trees[x][i] >= compareTree) {
                return false;
            }
        }
        return true;
    }

    public static boolean checkSouth(int[][] trees, int x, int y) {
        int compareTree = trees[x][y];
        for (int i = y + 1; i < 99; i++) {
            if (trees[x][i] >= compareTree) {
                return false;
            }
        }
        return true;
    }

    private static int getHighestScenicScore(int[][] trees) {
        int highestScore = 0;
        for (int x = 0; x < 99; x++) {
            for (int y = 0; y < 99; y++) {
                int scenicScore = getScenicScore(trees, x, y);
                if (scenicScore > highestScore) {
                    highestScore = scenicScore;
                }
            }
        }
        return highestScore;
    }

    private static int getScenicScore(int[][] trees, int x, int y) {
        return getScenicWest(trees, x, y) * getScenicEast(trees, x, y) *
                getScenicNorth(trees, x, y) * getScenicSouth(trees, x, y);
    }

    private static int getScenicWest(int[][] trees, int x, int y) {
        int points = 0;
        int compareTree = trees[x][y];
        for (int i = x - 1; i >= 0; i--) {
            points++;
            if (trees[i][y] >= compareTree) {
                break;
            }
        }
        return points;
    }

    private static int getScenicEast(int[][] trees, int x, int y) {
        int points = 0;
        int compareTree = trees[x][y];
        for (int i = x + 1; i < 99; i++) {
            points++;
            if (trees[i][y] >= compareTree) {
                break;
            }
        }
        return points;
    }

    private static int getScenicNorth(int[][] trees, int x, int y) {
        int points = 0;
        int compareTree = trees[x][y];
        for (int i = y - 1; i >= 0; i--) {
            points++;
            if (trees[x][i] >= compareTree) {
                break;
            }
        }
        return points;
    }

    private static int getScenicSouth(int[][] trees, int x, int y) {
        int points = 0;
        int compareTree = trees[x][y];
        for (int i = y + 1; i < 99; i++) {
            points++;
            if (trees[x][i] >= compareTree) {
                break;
            }
        }
        return points;
    }
}
