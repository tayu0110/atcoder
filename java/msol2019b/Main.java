import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    String s = sc.next();
    int ans = 0;
    for(int i = 0; i < s.length(); i++) {
      if(s.charAt(i) == 'x') ans++;
    }
    if(ans > 7) System.out.println("NO");
    else System.out.println("YES");
  }
}