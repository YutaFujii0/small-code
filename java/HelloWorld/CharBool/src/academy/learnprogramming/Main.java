package academy.learnprogramming;

public class Main {
    public static void main(String[] args) {
        char myChar = 'D';
        char myUnicodeChar = '\u0044';
        System.out.println(myChar);
        System.out.println(myUnicodeChar);
        char myCopyrightChar = '\u00A9';
        System.out.println(myCopyrightChar);

        boolean myTrueBooleanValue = true;
        boolean myFalseBooleanValue = false;

        // primitive types
        // byte 8bit
        // short 16
        // int 32
        // long 64
        // float 32
        // double 64
        // char 16
        // boolean 1bit

        String myString = "This is a string";
        System.out.println(myString);

        String numberString = "250.55";
        numberString += "49.95";
        System.out.println(numberString);

        String lastString = "10";
        int myInt = 50;
        lastString += myInt; // warning but still works
        System.out.println("LastString is equal to " + lastString);
    }
}
