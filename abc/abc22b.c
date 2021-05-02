#include<stdio.h>
#include<string.h>
#include<stdbool.h>
#define MAX_N 100005
int main() {
  int n;
  scanf("%d", &n);
  int a[MAX_N];
  bool ck[MAX_N];
  memset(ck, false, sizeof(ck));
  for(int i = 0; i < n; i++) {
    scanf("%d", a + i);
  }
  int ans = 0;
  for(int i = 0; i < n; i++) {
    if(ck[a[i]]) ans++;
    else ck[a[i]] = true;
  }
  printf("%d\n", ans);
  return 0;
}