#include<stdio.h>
#define LEN 105
int main() {
  int n;
  scanf("%d", &n);
  char c[LEN];
  scanf("%s", c);
  int ans[4];
  for(int i = 0; i < 4; i++) ans[i] = 0;
  for(int i = 0; i < n; i++) {
    int k = c[i] - '1';
    ans[k]++;
  }
  int mx = 0, mn = 10000;
  for(int i = 0; i < 4; i++) {
    if(mx < ans[i]) mx = ans[i];
    if(mn > ans[i]) mn = ans[i];
  }
  printf("%d %d\n", mx, mn);
  return 0;
}