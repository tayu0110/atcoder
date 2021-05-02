#include<stdio.h>
#define MAX_N 105
int main() {
  int n;
  scanf("%d", &n);
  int a[MAX_N];
  for(int i = 0; i < n; i++) {
    scanf("%d", a + i);
  }
  int r = 1;
  int ans = 0;
  for(int i = 1; i < n; i++) {
    if(a[i] == a[i-1]) r++;
    else {
      ans += r/2;
      r = 1;
    }
  }
  ans += r/2;
  printf("%d\n", ans);
  return 0;
}