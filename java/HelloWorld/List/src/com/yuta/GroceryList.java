package com.yuta;

import java.util.ArrayList;

public class GroceryList {
    private int[] myNumbers = new int[50];
    private ArrayList<String> groceryList = new ArrayList<String>();

    public void addGroceryItem(String item) {
        groceryList.add(item);
    }

    public void modifyGroceryItem(int index, String newItem) {
        groceryList.set(index, newItem);
    }

    public void removeGroceryItem(int index) {
        groceryList.remove(index);
    }

    public String findItem(String searchItem) {
        // boolean exists = groceryList.contains(searchItem);
        int index = groceryList.indexOf(searchItem);
        if (position >= 0) {
            return groceryList.get(index);
        }

        return null;
    }
}
