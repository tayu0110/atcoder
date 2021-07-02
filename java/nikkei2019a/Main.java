import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int n = Integer.parseInt(sc.next());
    int a = Integer.parseInt(sc.next());
    int b = Integer.parseInt(sc.next());
    int mx = Math.min(a, b);
    int mn = Math.max(0, (a+b) - n);
    System.out.printf("%d %d\n", mx, mn);
  }
}