import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Scanner;
import java.util.concurrent.atomic.AtomicInteger;

public class day1 {
    public static void main(String[] args) {
        try {
            File myObj = new File("day1input");
            Scanner myReader = new Scanner(myObj);
            List<AtomicInteger> calories = new ArrayList<>();
            calories.add(new AtomicInteger(0));
            while (myReader.hasNextLine()) {
                String line = myReader.nextLine();
                if(line.equals("")) {
                    calories.add(new AtomicInteger(0));
                } else {
                    calories.get(calories.size()-1).getAndAdd(Integer.parseInt(line));
                }
            }
            myReader.close();
            calories.sort(Comparator.comparingInt(AtomicInteger::get));
            //Part 1
            System.out.println(calories.get(calories.size()-1));

            //Part 2
            System.out.println(calories.get(calories.size()-1).get() + calories.get(calories.size()-2).get() + calories.get(calories.size()-3).get());
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }

    }
}
