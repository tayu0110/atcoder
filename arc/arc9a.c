#include<stdio.h>
#define MAX_N 15
int main() {
  int n;
  scanf("%d", &n);
  int ans = 0;
  for(int i = 0; i < n; i++) {
    int a, b;
    scanf("%d%d", &a, &b);
    ans += a * b * 105;
  }
  ans /= 100;
  printf("%d\n", ans);
  return 0;
}