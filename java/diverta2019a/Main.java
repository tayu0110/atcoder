import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    int n, k;
    Scanner sc = new Scanner(System.in);
    n = Integer.parseInt(sc.next());
    k = Integer.parseInt(sc.next());
    System.out.println(n-k+1);
  }
}
