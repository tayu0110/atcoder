import java.util.HashSet;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    char c = sc.next().charAt(0);
    if(c == 'K' || c == 'L' || c == 'O' || c == 'P') System.out.println("Right");
    else System.out.println("Left");
  }
}