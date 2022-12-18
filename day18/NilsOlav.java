import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class day18 {


    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    private static void part2() throws FileNotFoundException {
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day18input");
        Scanner myReader = new Scanner(myObj);
        List<Cube> cubes = new ArrayList<>();
        int largestX = 0;
        int largestY = 0;
        int largestZ = 0;
        //int numberOfEdges = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            Cube cube = getCube(line);
            //numberOfEdges += getNumberOfEdges(cube, cubes);
            cubes.add(cube);
            if (cube.x > largestX) {
                largestX = cube.x;
            }

            if (cube.y > largestY) {
                largestY = cube.y;
            }
            if (cube.z > largestZ) {
                largestZ = cube.z;
            }
        }

        List<Cube> airCubes = new ArrayList<>();
        for (int i = 0; i < largestX; i++) {
            for (int j = 0; j < largestY; j++) {
                for (int k = 0; k < largestZ; k++) {
                    Cube cube = new Cube(i, j, k);
                    if (!cubes.contains(cube)) {
                        airCubes.add(cube);
                    }
                }
            }
        }

        while (removeAirCubesTouchingAir(airCubes, cubes) > 0) {}

        cubes.addAll(airCubes);

        int numberOfEdges = 0;
        for(Cube c : cubes) {
            numberOfEdges += getNumberOfEdges(c, cubes);
        }

        System.out.println("Edges: " + numberOfEdges);
        System.out.println("Cubes: " + cubes.size());
        int surfaceArea = cubes.size() * 6 - numberOfEdges;
        System.out.println("Surface area: " + surfaceArea);

    }

    public static void part1() throws FileNotFoundException {
        File myObj = new File("/Users/meddan/Documents/AdventOfCode2022/resources/day18input");
        Scanner myReader = new Scanner(myObj);
        List<Cube> cubes = new ArrayList<>();
        int numberOfEdges = 0;
        while (myReader.hasNextLine()) {
            String line = myReader.nextLine();
            Cube cube = getCube(line);
            numberOfEdges += getNumberOfEdges(cube, cubes);
            cubes.add(cube);
        }

        System.out.println("Edges: " + numberOfEdges);
        System.out.println("Cubes: " + cubes.size());
        int surfaceArea = cubes.size() * 6 - numberOfEdges * 2;
        System.out.println("Surface area: " + surfaceArea);
    }

    public static int removeAirCubesTouchingAir(List<Cube> airCubes, List<Cube> realCubes) {
        List<Cube> toRemove = new ArrayList<>();
        for (Cube air : airCubes) {
            int realNeighbours = getNumberOfEdges(air, realCubes);
            int airNeighbours = getNumberOfEdges(air, airCubes);
            //System.out.println(air + " real: " + realEdges + " air: " + airEdges);
            if (realNeighbours + airNeighbours != 6) {
                toRemove.add(air);
            }
        }
        airCubes.removeAll(toRemove);
        return toRemove.size();
    }
    public static int getNumberOfEdges(Cube cube, List<Cube> cubes) {
        int edges = 0;
        for (Cube test : cubes) {
            if (sharesEdge(cube, test)) {
                edges++;
            }
        }
        return edges;
    }

    private static boolean sharesEdge(Cube c1, Cube c2) {
        return Math.abs(c1.x - c2.x) + Math.abs(c1.y - c2.y) + Math.abs(c1.z - c2.z) == 1;
    }

    public static Cube getCube(String line) {
        String[] split = line.split(",");
        return new Cube(split[0], split[1], split[2]);
    }

    public static class Cube {
        public int x;
        public int y;
        public int z;

        public Cube(int x, int y, int z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public Cube(String x, String y, String z) {
            this.x = Integer.parseInt(x);
            this.y = Integer.parseInt(y);
            this.z = Integer.parseInt(z);
        }


        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Cube cube = (Cube) o;
            return x == cube.x && y == cube.y && z == cube.z;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y, z);
        }

        @Override
        public String toString() {
            return x + "," + y + "," + z;
        }
    }
}
