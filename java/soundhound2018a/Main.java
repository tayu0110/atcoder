import java.util.Scanner;
public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int a = Integer.parseInt(sc.next());
    int b = Integer.parseInt(sc.next());
    if(a + b == 15) System.out.println("+");
    else if(a * b == 15) System.out.println("*");
    else System.out.println("x");
  }
}
