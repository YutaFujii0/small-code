package com.yuta;

import java.io.*;
import java.util.*;

public class Locations implements Map<Integer, Location> {
    private static Map<Integer, Location> locations = new HashMap<>();

    public static void main(String[] args) throws IOException {
//        try (DataOutputStream locFile = new DataOutputStream(new BufferedOutputStream(new FileOutputStream("locations.dat")))) {
//            for (Location location : locations.values()) {
//                locFile.writeInt(location.getLocationID());
//                locFile.writeUTF(location.getDescription());
//                locFile.writeInt(location.getExits().size() - 1);
//                for (String direction : location.getExits().keySet()) {
//                    if (!direction.equalsIgnoreCase("Q")) {
//                        locFile.writeUTF(direction);
//                        locFile.writeInt(location.getExits().get(direction));
//                    }
//                }
//            }
//        }
//        Map<String, Integer> exits = new HashMap<>();
//        Location location = new Location(0,"You are sitting in front of a computer learning Java", exits);
//        locations.put(location.getLocationID(), location);
//
//        exits = new HashMap<>();
//        exits.put("W",2);
//        exits.put("E",3);
//        exits.put("S",4);
//        exits.put("N",5);
//        location = new Location(1,"You are standing at the end of a road before a small brick building", exits);
//        locations.put(location.getLocationID(), location);
//
//        exits = new HashMap<>();
//        exits.put("N",5);
//        location = new Location(2,"You are at the top of a hill", exits);
//        locations.put(location.getLocationID(), location);
//
//        exits = new HashMap<>();
//        exits.put("W",1);
//        location = new Location(3,"You are inside a building, a well house for a small spring", exits);
//        locations.put(location.getLocationID(), location);
//
//        exits = new HashMap<>();
//        exits.put("N",1);
//        exits.put("W",2);
//        location = new Location(4,"You are in a valley beside a stream", exits);
//        locations.put(location.getLocationID(), location);
//
//        exits = new HashMap<>();
//        exits.put("S",1);
//        exits.put("W",2);
//        location = new Location(5,"You are in the forest", exits);
//        locations.put(location.getLocationID(), location);
//

        try (ObjectOutputStream locFile = new ObjectOutputStream(new BufferedOutputStream(new FileOutputStream("locations2.dat")))) {
            for (Location loc : locations.values()) {
                locFile.writeObject(loc);
            }
        }
    }

    static {
//        try (DataInputStream locFile = new DataInputStream(new BufferedInputStream(new FileInputStream("locations.dat")))) {
        try (ObjectInputStream locFile = new ObjectInputStream(new BufferedInputStream(new FileInputStream("locations.dat")))) {
            boolean eof = false;
            while (!eof) {
                try {
                    Location location = (Location) locFile.readObject();
                    System.out.println("imported location " + location.getLocationID());
                    locations.put(location.getLocationID(), location);
                } catch (EOFException e) {
                    eof = true;
                }
            }
//            while (!eof) {
//                try {
//                    int loc = locFile.readInt();
//                    String description = locFile.readUTF();
//                    Location location = new Location(loc, description);
//                    int num = locFile.readInt();
//                    for (int i=0; i<num; i++) {
//                        String direction = locFile.readUTF();
//                        int dest = locFile.readInt();
//                        location.addExit(direction, dest);
//                    }
//                    locations.put(loc, location);
//                } catch (EOFException e) {
//                    eof = true;
//                }
//            }
        } catch (IOException e) {
            System.out.println("IO Exception");
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFound exception");
        }
    }

//    static {
////         scanner = null;
//        try (Scanner scanner = new Scanner(new FileReader("locations.txt"));) {
//
//            scanner.useDelimiter(",");
//            while (scanner.hasNextLine()) {
//                int loc = scanner.nextInt();
//                scanner.skip(scanner.delimiter());
//                String description = scanner.nextLine();
//                System.out.println("Imported loc: " + loc + ": " + description);
//                locations.put(loc, new Location(loc, description));
//            }
//        } catch (IOException e) {
//            e.printStackTrace();
//        }
//
//        // try with resources `try ()` automatically handle this
////        } finally {
////            if (scanner != null) {
////                scanner.close();
////            }
////        }
//
//        try (BufferedReader dirFile = new BufferedReader(new FileReader("directions.txt"))) {
//            String input;
//            while ((input = dirFile.readLine()) != null) {
//                String data[] = input.split(",");
//                int loc = Integer.parseInt(data[0]);
//                int destination = Integer.parseInt(data[2]);
//                Location location = locations.get(loc);
//                location.addExit(data[1], destination);
//            }
//        } catch (IOException e) {
//            e.printStackTrace();
//        }
//    }

    @Override
    public int size() {
        return locations.size();
    }

    @Override
    public boolean isEmpty() {
        return locations.isEmpty();
    }

    @Override
    public boolean containsKey(Object key) {
        return locations.containsKey(key);
    }

    @Override
    public boolean containsValue(Object value) {
        return locations.containsValue(value);
    }

    @Override
    public Location get(Object key) {
        return locations.get(key);
    }

    @Override
    public Location put(Integer key, Location value) {
        return locations.put(key, value);
    }

    @Override
    public Location remove(Object key) {
        return locations.remove(key);
    }

    @Override
    public void putAll(Map<? extends Integer, ? extends Location> m) {
        locations.putAll(m);
    }

    @Override
    public void clear() {
        locations.clear();
    }

    @Override
    public Set<Integer> keySet() {
        return locations.keySet();
    }

    @Override
    public Collection<Location> values() {
        return locations.values();
    }

    @Override
    public Set<Entry<Integer, Location>> entrySet() {
        return locations.entrySet();
    }
}
