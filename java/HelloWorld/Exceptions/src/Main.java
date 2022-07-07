import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        System.out.println(getIntEAFP());
    }

    private static int getIntEAFP() {
        Scanner s = new Scanner(System.in);
        return s.nextInt();
//        try {
//            return s.nextInt();
//        } catch (InputMismatchException e) {
//            return 0;
//        }
    }
}
