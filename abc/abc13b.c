#include<stdio.h>

int main() {
  int a, b;
  scanf("%d%d", &a, &b);
  if(a < b) {
    int tmp = a;
    a = b;
    b = tmp;
  }
  int ans = a - b;
  if(ans > 5) ans = 10 - ans;
  printf("%d\n", ans);
  return 0;
}
