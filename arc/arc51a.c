#include<stdio.h>
#include<stdbool.h>
bool isInside(int a, int b, int x, int y, int r) {
  // printf("%d %d %d %d %d %d\n", a, b, x, y, r, (a - x) * (a - x) + (b - y) * (b - y));
  return (a - x) * (a - x) + (b - y) * (b - y) <= r * r;
}
int main() {
  int x1, y1, r;
  scanf("%d%d%d", &x1, &y1, &r);
  int x2, y2, x3, y3;
  scanf("%d%d%d%d", &x2, &y2, &x3, &y3);
  bool f1, f2, f3, f4;
  f1 = isInside(x2, y3, x1, y1, r);
  f2 = isInside(x2, y3, x1, y1, r);
  f3 = isInside(x3, y2, x1, y1, r);
  f4 = isInside(x3, y3, x1, y1, r);
  // printf("%d %d %d %d\n", f1, f2, f3, f4);
  if(f1 && f2 && f3 && f4) printf("YES NO\n");
  else if(x1 - r >= x2 && x1 + r <= x3 && y1 - r >= y2 && y1 + r <= y3) printf("NO YES\n");
  else printf("YES YES\n");
  return 0;
}