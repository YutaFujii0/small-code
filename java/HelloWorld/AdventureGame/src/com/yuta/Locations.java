package com.yuta;

import java.io.FileReader;
import java.io.IOException;
import java.util.*;

public class Locations implements Map<Integer, Location> {
    private static Map<Integer, Location> locations = new HashMap<>();

    static {
        Scanner scanner = null;
        try {
            scanner = new Scanner(new FileReader("locations.txt"));
        } catch (IOException e) {
            e.printStackTrace();
        } finally {
            if (scanner != null) {
                scanner.close();
            }
        }

//        locations.get(1).addExit("W", 2);
//        locations.get(1).addExit("E", 3);
//        locations.get(1).addExit("S", 4);
//        locations.get(1).addExit("N", 5);
//
//        locations.get(2).addExit("N", 5);
//
//        locations.get(3).addExit("W", 1);
//
//        locations.get(4).addExit("N", 1);
//        locations.get(4).addExit("W", 2);
//
//        locations.get(5).addExit("S", 1);
//        locations.get(5).addExit("W", 2);
    }

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
