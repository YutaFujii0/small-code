package com.yuta;

import java.util.Arrays;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Iterator;

public class Main {
    public static void main(String[] args) {
        int[] myIntArray = new int[10];
        myIntArray[0] = 1;
        myIntArray[1] = 10;
        myIntArray[3] = 1222;

        int[] myAnotherIntArray = {1,2,3,4,5,6,7,8,9,10};
        int[] anotherInitialization = new int[]{3,5,3};

        // reference types vs value types
        int number = 1;
        int anotherNumber = number;

        System.out.println("number = " + number);
        System.out.println("anotherNumber = " + anotherNumber);

        anotherNumber++;

        System.out.println("number = " + number);
        System.out.println("anotherNumber = " + anotherNumber);

        modifyInt(anotherNumber);

        System.out.println("number = " + number);
        System.out.println("anotherNumber = " + anotherNumber);

        int[] someArray = new int[5];
        int[] anotherArray = someArray;

        System.out.println("someArray = " + Arrays.toString(someArray));
        System.out.println("anotherArray = " + Arrays.toString(anotherArray));

        anotherArray[0] = 1;

        System.out.println("someArray = " + Arrays.toString(someArray));
        System.out.println("anotherArray = " + Arrays.toString(anotherArray));

        modifyArray(someArray);

        System.out.println("someArray = " + Arrays.toString(someArray));
        System.out.println("anotherArray = " + Arrays.toString(anotherArray));

        // autoboxing an unboxing
        // cannot use primitive type cause it's not an object of a class
        // ArrayList<int> intArray = new ArrayList<int>();
        ArrayList<Integer> intArray = new ArrayList<Integer>();
        for(int i=0; i<10; i++) {
            intArray.add(Integer.valueOf(i)); // precise statement
            intArray.add(i); // autoboxing still works
        }
        for(int i=0; i<10; i++) {
            System.out.println(i + " --> " + intArray.get(i).intValue()); // precise statement
            System.out.println(i + " --> " + intArray.get(i)); // unboxing
        }

        // Linked list
        LinkedList<String> placeToVisit = new LinkedList<String>();
        placeToVisit.add("Tokyo");
        placeToVisit.add("New York");
        placeToVisit.add("Seoul");
        placeToVisit.add("Hong Kong");
        placeToVisit.add("Madrid");
        placeToVisit.add("Sao Paolo");
        placeToVisit.add("San Jose");

        printList(placeToVisit);
    }

    public static void modifyInt(int number) {
        number++;
    }

    public static void modifyArray(int[] array) {
        array[0] = 2;
    }

    public static void printList(LinkedList<String> linkedList) {
        Iterator<String> i = linkedList.iterator();
        while(i.hasNext()) {
            System.out.println("Place to visit " + i.next());
        }
        System.out.println("----------------");
    }
}
