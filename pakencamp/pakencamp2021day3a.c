#include <stdio.h>

int main() {
  int a, b, c, d;
  scanf("%d %d %d %d", &a, &b, &c, &d);
  int ans = (a == 1111) + (b == 1111) + (c == 1111) + (d == 1111);
  printf("%d\n", ans);
  return 0;
}