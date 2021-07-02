public class Main {
  public static void main(String[] args) {
    long ans = 42;
    while(ans < 130000000) ans *= 2L;
    System.out.println(ans);
  }
}