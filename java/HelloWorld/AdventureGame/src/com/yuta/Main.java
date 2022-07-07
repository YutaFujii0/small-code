package com.yuta;

public class Main {
    private static Locations locations = new Locations();

    public static void main(String[] args) {
        try {
            System.out.println(locations.get(1).getDescription());
        } catch (NullPointerException e) {
            System.out.println("Detected null pointer exception.");
        }
    }

    public void command() {
        // write code here
    }
}