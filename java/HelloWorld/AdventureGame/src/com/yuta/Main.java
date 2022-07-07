package com.yuta;

public class Main {
    private static Locations locations = new Locations();

    public static void main(String[] args) {
        System.out.println(locations.get(1).getDescription());
    }

    public void command() {
        // write code here
    }
}