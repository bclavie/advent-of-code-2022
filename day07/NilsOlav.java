import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.stream.Collectors;

public class day7 {

    public static void main(String[] args) throws FileNotFoundException {
        System.out.println("part1 " + part1());
        System.out.println("part2 " + part2());
    }

    private static int part1() throws FileNotFoundException {
        return traverseTree(buildTree());
    }

    private static int part2() throws FileNotFoundException {
        List<Integer> sizeOfDirs = getSizeOfDirs(buildTree());
        Optional<Integer> max = sizeOfDirs.stream().max(Integer::compareTo);
        int minSize = 70000000 - 46975962;
        int used = max.get();
        int fileSystem = 70000000;
        int updateSize = 30000000;
        int spaceToClear = used - (fileSystem - updateSize);
        int smallest = Integer.MAX_VALUE;
        for (int i : sizeOfDirs) {
            if (i >= spaceToClear && i < smallest) {
                smallest = i;
            }
        }
        return smallest;
    }

    private static Directory buildTree() throws FileNotFoundException {
        File myObj = new File("day7input");
        Scanner myReader = new Scanner(myObj);
        Directory root = new Directory(null);
        Directory currentDirectory = root;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            if (line.equals("$ ls")) {
                //Do nothing?
            } else if (line.startsWith("dir")) {
                String dirName = line.split("dir ")[1];
                currentDirectory.getNodes().put(dirName, new Directory(currentDirectory));
            } else if (line.equals("$ cd ..")) {
                currentDirectory = currentDirectory.getParent();
            } else if (line.startsWith("$ cd")) {
                String dir = line.split("cd ")[1];
                currentDirectory = (Directory) currentDirectory.getNodes().get(dir);
            } else {
                String[] file = line.split(" ");
                currentDirectory.getNodes().put(file[1], new FileNode(Integer.parseInt(file[0])));
            }
        }
        return root;
    }

    private static List<Integer> getSizeOfDirs(Node node) {
        //24933642
        if (node instanceof Directory dir) {
            List<Integer> sizes = new ArrayList<>();
            for (Node n : dir.nodes.values()) {
                sizes.addAll(getSizeOfDirs(n));
            }
            sizes.add(dir.getSize());
            return sizes;
        }

        return List.of();
    }

    private static int traverseTree(Node node) {
        if (node instanceof Directory dir) {
            int sum = 0;
            for (Node n : dir.getNodes().values()) {
                sum += traverseTree(n);
            }
            if (dir.getSize() <= 100000) {
                sum += dir.getSize();
            }
            return sum;
        }
        return 0;
    }

    interface Node {
        int getSize();
    }

    static class Directory implements Node {
        private Map<String, Node> nodes;
        private Directory parent;

        public Directory(Directory parent) {
            this.parent = parent;
            this.nodes = new HashMap<>();
        }

        public Map<String, Node> getNodes() {
            return nodes;
        }

        public Directory getParent() {
            return parent;
        }

        @Override
        public int getSize() {
            int sum = 0;
            for (Node n : nodes.values()) {
                sum += n.getSize();
            }
            return sum;
        }
    }

    static class FileNode implements Node {

        private int size;

        public FileNode(int size) {
            this.size = size;
        }

        @Override
        public int getSize() {
            return size;
        }
    }
}
