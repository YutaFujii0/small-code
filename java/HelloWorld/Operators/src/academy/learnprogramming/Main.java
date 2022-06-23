package academy.learnprogramming;

public class Main {

    public static void main(String[] args) {
        int result = 1 + 2;
        System.out.println("1 + 2 = " + result);
        int previousResutl = result;
        System.out.println("PreviousResult = " + previousResutl);
        result = result - 1;
        System.out.println("3 - 1 = " + result);
        System.out.println("PreviousResult = " + previousResutl);

        result *= 10;
        System.out.println("2 * 10 = " + result);

        result /= 5;
        System.out.println("20 / 5 = " + result);

        result %= 3;
        System.out.println("4 % 3 = " + result);

        boolean isAlien = false;
        if (isAlien == false); // this is still valid and does nothing
            System.out.println("It is not an alien!"); // this is always executed

        if (isAlien == true)
            System.out.println("You will not see this!");
            System.out.println("But you'll see this!");

        if (isAlien == false) {
            System.out.println("You will see this!");
            System.out.println("And you'll see this!");
        }

        int topScore = 100;
        if (topScore == 100) {
            System.out.println("Y");
        }
    }
}
