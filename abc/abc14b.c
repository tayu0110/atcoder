#include<stdio.h>
#define MAX_N 25
int main() {
  int n, x;
  scanf("%d%d", &n, &x);
  int a[MAX_N];
  int ans = 0;
  for(int i = 0; i < n; i++) {
    scanf("%d", a + i);
    if((1 << i) & x) ans += a[i];
  }
  printf("%d\n", ans);
  return 0;
}