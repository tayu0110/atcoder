import java.util.Scanner;
public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    int n = Integer.parseInt(sc.next());
    String s = sc.next();
    int r = 0, b = 0;
    for(int i = 0; i < s.length(); i++) {
      if(s.charAt(i) == 'R') r++;
      else b++;
    }
    if(r > b) System.out.println("Yes");
    else System.out.println("No");
  }
}

