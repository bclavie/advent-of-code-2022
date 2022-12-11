import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.concurrent.atomic.AtomicInteger;

public class day10 {

    public static void main(String[] args) throws FileNotFoundException {
        System.out.println("part1 " + part1());
    }

    public static int part1() throws FileNotFoundException {
        File myObj = new File("day10input");
        Scanner myReader = new Scanner(myObj);
        List<Integer> values = new ArrayList<>();
        AtomicInteger cycle = new AtomicInteger(0);
        List<Boolean> pixels = new ArrayList<>();
        int register = 1;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            System.out.println(line);
            String[] operation = line.split(" ");
            if (operation[0].equals("noop")) {
                pixels.add(incrementCycle(cycle, register, values));
            } else {
                pixels.add(incrementCycle(cycle, register, values));
                pixels.add(incrementCycle(cycle, register, values));
                register += Integer.parseInt(operation[1]);
            }
        }
        int sum = 0;
        for (Integer i : values) {
            sum += i;
        }
        String s = new String();
        for (int i = 0; i < pixels.size(); i++) {
            boolean b = pixels.get(i);
            if (i % 40 == 0) {
                s += "\n";
            }
            if (b) {
                s += "#";
            } else {
                s += ".";
            }

        }
        System.out.println(s);

        return sum;
    }

    public static boolean incrementCycle(AtomicInteger cycle, int registry, List<Integer> values) {
        cycle.incrementAndGet();
        if ((cycle.get() - 20) % 40 == 0) {
            values.add(registry * cycle.get());
        }

        int position = (cycle.get() % 40) - 1;
        return registry - 1 == position || registry == position || registry + 1 == position;
    }

}
