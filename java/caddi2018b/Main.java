import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int n = Integer.parseInt(sc.next());
    long h = Integer.parseInt(sc.next());
    long w = Integer.parseInt(sc.next());
    int ans = 0;
    for(int i = 0; i < n; i++) {
      long a = Integer.parseInt(sc.next());
      long b = Integer.parseInt(sc.next());
      if(a >= h && b >= w) ans++;
    }
    System.out.println(ans);
  }
}