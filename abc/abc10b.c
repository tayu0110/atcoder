#include<stdio.h>
#define MAX_N 15
int main() {
  int n;
  scanf("%d", &n);
  int a[MAX_N];
  int ans = 0;
  for(int i = 0; i < n; i++) {
    scanf("%d", a+i);
    if(a[i] == 9) continue;
    else if(a[i] == 8) ans++;
    else if(a[i] == 7) continue;
    else if(a[i] > 3) ans += a[i]-3;
    else if(a[i] == 3) continue;
    else if(a[i] == 2) ans++;
    else continue; 
  }
  printf("%d\n", ans);
  return 0;
}