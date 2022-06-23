import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        boolean gameOver = true;
        int score = 800;
        int levelCompleted = 5;
        int bonus = 100;

        int highScore = calculateScore(gameOver, score, levelCompleted, bonus);
        System.out.println("Your final score was " + highScore);

        gameOver = true;
        score = 800;
        levelCompleted = 5;
        bonus = 100;

        highScore = calculateScore(gameOver, score, levelCompleted, bonus);
        System.out.println("Your final score was " + highScore);

        // method overloading
        // ex) println method is defined 10 times for a variety of types
        calculateScore("Bob", 100);
        // calculateScore(100, 100);
        calculateScore(100);

        // switch statement
        int switchValue = 2;
        switch(switchValue) {
            case 1:
                System.out.println("Value was 1");
                break;

            case 2:
                System.out.println("Value was 2");
                break;

            case 3: case 4: case 5:
                System.out.println("Value was either 3 or 4 or 5");
                break;

            default:
                System.out.println("Value was 1");
                break;
        }

        // loop
        for(int i=0; i < 10; i++) {
            double interest = 10000d * i / 100;
            System.out.println("Interest is " + interest);
        }

        int count = 0;
        int fibonnacci1 = 0;
        int fibonnacci2 = 1;
        while(count < 10) {
            int tmp = fibonnacci1 + fibonnacci2;
            fibonnacci1 = fibonnacci2;
            fibonnacci2 = tmp;

            System.out.println("fibonnacci = " + fibonnacci2);
            count ++;
        }

        // parse string
        String numberAsString = "2022";
        int number = Integer.parseInt(numberAsString);
        System.out.println(number);

        // user input
        // Scanner scanner = new Scanner(System.in);
        // System.out.println("Enter your name: ");
        // String name = scanner.nextLine();
        // scanner.nextLine()
        // String name = scanner.nextInt();
        // System.out.println("Your name is " + name);
        // scanner.close();
    }

    public static void displayHighScorePosition(String playerName, int highScorePosition) {
        System.out.println(playerName + " managed to get into position "
                + highScorePosition + " on the high score table");
    }

    public static int calculateScore(boolean gameOver, int score, int levelCompleted, int bonus) {
        if (gameOver) {
            int finalScore = score + (levelCompleted * bonus);
            finalScore += 1000;
            return finalScore;
        }
        return -1;
    }

    // method overloading
    public static int calculateScore(String playerName, int score) {
        System.out.println("Player " + playerName);
        return score * 1000;
    }

    public static int calculateScore(int score) {
        return score * 1000;
    }

    // does not recognize separate method only if the return type differs
//    public static void calculateScore(int score) {
//        System.out.println("calculated");
//    }
}
