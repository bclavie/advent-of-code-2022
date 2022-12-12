import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day12 {
    public static void main(String[] args) throws FileNotFoundException {
        part1();
    }

    public static void part1() throws FileNotFoundException {
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day12input");
        Scanner myReader = new Scanner(myObj);
        char[][] nodes = new char[161][41];
        int y = 0;
        //Create nodes
        Node start = null;
        Node end = null;
        Map<Node, Integer> visitList = new HashMap<>();
        while (myReader.hasNextLine()) {
            int x = 0;
            String line = myReader.next();
            char[] chars = line.toCharArray();
            for (char c : chars) {
                if (c == 'S') {
                    start = new Node(x, y);
                    visitList.put(start, 0);
                } else if (c == 'a') { //This is part 2
                    visitList.put(new Node(x, y), -1);
                } else if (c == 'E') {
                    end = new Node(x, y);
                    visitList.put(end, Integer.MAX_VALUE);
                } else {
                    visitList.put(new Node(x, y), Integer.MAX_VALUE);
                }
                nodes[x][y] = c;
                x++;
            }
            y++;
        }
        //Start visiting
        visitNode(nodes, start, visitList);

        System.out.println(visitList.get(end));
    }

    public static void visitNode(char[][] nodes, Node node, Map<Node, Integer> visitList) {
        Integer currentCost = visitList.get(node);
        List<Node> neighbours = getNeighbours(nodes, node);
        for (Node neighbour : neighbours) {
            if (visitList.get(neighbour) > currentCost + 1) {
                visitList.put(neighbour, currentCost + 1);
                visitNode(nodes, neighbour, visitList);
            } else if (visitList.get(neighbour) == -1) {
                visitList.put(neighbour, 0);
                visitNode(nodes, neighbour, visitList);
            }
        }
    }

    public static List<Node> getNeighbours(char[][] nodes, Node node) {
        List<Node> neighbours = new ArrayList<>();
        int i = node.x;
        int j = node.y;
        char from = nodes[i][j];
        //Add above
        if (j != 0) {
            char to = nodes[i][j - 1];
            if (canAddEdge(from, to)) {
                neighbours.add(new Node(i, j - 1));
            }
        }
        //Add below
        if (j != nodes[i].length - 1) {
            char to = nodes[i][j + 1];
            if (canAddEdge(from, to)) {
                neighbours.add(new Node(i, j + 1));
            }
        }
        //Add left
        if (i != 0) {
            char to = nodes[i - 1][j];
            if (canAddEdge(from, to)) {
                neighbours.add(new Node(i - 1, j));
            }
        }
        //Add right
        if (i != nodes.length - 1) {
            char to = nodes[i + 1][j];
            if (canAddEdge(from, to)) {
                neighbours.add(new Node(i + 1, j));
            }
        }

        return neighbours;
    }

    public static boolean canAddEdge(char from, char to) {
        if (from == 'S') {
            from = 'a';
        } else if (from == 'E') {
            from = 'z';
        }

        if (to == 'S') {
            to = 'a';
        } else if (to == 'E') {
            to = 'z';
        }

        return from + 1 >= to;
    }

    public static class Node {
        public final int x;
        public final int y;

        public Node(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Node node = (Node) o;
            return x == node.x && y == node.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }

        @Override
        public String toString() {
            return "{" + "x=" + x + ", y=" + y + '}';
        }
    }
}
