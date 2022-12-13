import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Scanner;

public class day13 {

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part1() throws FileNotFoundException {
        List<List<Value>> values = parseValues();

        int sum = 0;
        for (int i = 0; i < values.size(); i++) {
            int compare = compare(values.get(i).get(0), values.get(i).get(1));
            if (compare <= 0) {
                if(compare == 0) {
                    System.out.println("zero is case");
                }
                sum += (i + 1);
            }
        }
        System.out.println(sum);
    }

    public static void part2() throws FileNotFoundException {
        List<Value> values = new ArrayList<>(parseValues().stream().flatMap(Collection::stream).toList());
        ValueList divider2 = new ValueList(List.of(new ValueList(List.of(new NumberValue(2)))));
        ValueList divider6 = new ValueList(List.of(new ValueList(List.of(new NumberValue(6)))));
        values.add(divider2);
        values.add(divider6);
        values.sort(day13::compare);

        System.out.println((values.indexOf(divider2) + 1) * (values.indexOf(divider6) + 1));
    }

    private static List<List<Value>> parseValues() throws FileNotFoundException {
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day13input");
        Scanner myReader = new Scanner(myObj);
        List<List<Value>> values = new ArrayList<>();
        List<Value> currentPair = new ArrayList<>();

        while (myReader.hasNext()) {
            String left = myReader.next();
            currentPair.add(parseValue(left));
            String right = myReader.next();
            currentPair.add(parseValue(right));
            values.add(currentPair);
            currentPair = new ArrayList<>();
        }

        return values;
    }

    public static int compare(Value left, Value right) {
        if (left instanceof ValueList && right instanceof NumberValue) {
            return compare(left, new ValueList(List.of(right)));
        } else if (left instanceof NumberValue && right instanceof ValueList) {
            return compare(new ValueList(List.of(left)), right);
        } else if (left instanceof ValueList && right instanceof ValueList) {
            for (int i = 0; i < ((ValueList) left).values.size(); i++) {
                if (i >= ((ValueList) right).values.size()) {
                    return 1;
                }
                int compare = compare(((ValueList) left).values.get(i), ((ValueList) right).values.get(i));
                if (compare != 0) {
                    return compare;
                }
            }
            return ((ValueList) left).values.size() - ((ValueList) right).values.size();
        } else if (left instanceof NumberValue && right instanceof NumberValue) {
            return ((NumberValue) left).value - ((NumberValue) right).value;
        } else {
            throw new RuntimeException("Unmatched comparison: " + left + " | " + right);
        }
    }

    private static Value parseValue(String line) {
        char c = line.toCharArray()[0];
        if (c == '[') {
            String substring = line.substring(1, line.lastIndexOf("]"));
            if (substring.length() == 0) {
                return new ValueList(List.of());
            }
            String[] split = substring.split(",");
            List<Value> parts = new ArrayList<>();
            for (int i = 0; i < split.length; i++) {
                if (unmatchedBraces(split[i])) {
                    String merge = split[i];
                    while (unmatchedBraces(merge)) {
                        i++;
                        merge += "," + split[i];
                    }
                    parts.add(parseValue(merge));
                } else {
                    parts.add(parseValue(split[i]));
                }
            }
            return new ValueList(parts);
        } else if (c == ']') {
            throw new RuntimeException("Encountered right brace!");
        } else {
            return new NumberValue(Integer.parseInt(line));
        }
    }

    private static boolean unmatchedBraces(String s) {
        int left = 0;
        int right = 0;
        for (char c : s.toCharArray()) {
            if (c == '[') {
                left++;
            }
            if (c == ']') {
                right++;
            }
        }
        if (right > left) {
            throw new RuntimeException("this should not happen: " + s);
        }
        return left != right;
    }

    public interface Value {
    }

    public static class NumberValue implements Value {
        public final int value;

        public NumberValue(int value) {
            this.value = value;
        }

        @Override
        public String toString() {
            return String.valueOf(value);
        }
    }

    public static class ValueList implements Value {
        public List<Value> values;

        public ValueList(List<Value> values) {
            this.values = values;
        }

        @Override
        public String toString() {
            return values.toString();
        }
    }
}
