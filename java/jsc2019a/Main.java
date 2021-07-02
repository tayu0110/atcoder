import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int m = Integer.parseInt(sc.next());
    int d = Integer.parseInt(sc.next());
    int ans = 0;
    for(int i = 1; i <= d; i++) {
      int t = i / 10;
      int o = i % 10;
      if(t < 2 || o < 2) continue;
      int k = t * o;
      if(k <= m && k > 0) ans++;
    }
    System.out.println(ans);
  }
}