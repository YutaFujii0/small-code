package com.yuta;

import java.util.HashMap;

public class Map {
    public static void main(String[] args) {
        // Map is also the name of this class, specify package
        // Map<String, String> languages = new HashMap<>();
        java.util.Map<String, String> languages = new HashMap<>();
        languages.put("Java", "a compiled high level, object-oriented platform independent language");
        languages.put("Python", "an interpreted, object-oriented high-level programming language");
        languages.put("BASIC", "Beginners All Purposes Instruction Code");

        System.out.println(languages.get("Java"));
        languages.containsKey("Java");
        languages.put("Java", "hi");

    }
}
