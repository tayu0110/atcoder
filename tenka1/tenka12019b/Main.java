import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int n = Integer.parseInt(sc.next());
    String s = sc.next();
    int k = Integer.parseInt(sc.next());
    char c = s.charAt(k-1);
    String ans = "";
    for(int i = 0; i < s.length(); i++) {
      if(s.charAt(i) != c) ans += "*";
      else ans += c;
    }
    System.out.println(ans);
  }
}
