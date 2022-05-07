#include <stdio.h>

int a[110], b[110];

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int main() {
  int n;
  scanf("%d", &n);
  for(int i=0;i<n;i++) scanf("%d", a+i);
  int mn = 1001001, mx = 0;
  for(int i=0;i<n;i++) {
    mn = min(mn, a[i]);
    mx = max(mx, a[i]);
    b[a[i]]++;
  }
  if(mn != (mx+1) / 2) {
    printf("Impossible\n");
    return 0;
  }
  if(mn == 1 && b[mn] > 1 && n > 2) {
    printf("Impossible\n");
    return 0;
  }
  for(int i=mn;i<=mx;i++) {
    if(i == mn) {
      if(mx % 2 == 0 && b[i] > 1) {
        printf("Impossible\n");
        return 0;
      }
      if(mx % 2 == 1 && b[i] > 2) {
        printf("Impossible\n");
        return 0;
      }
    } else {
      if(b[i] < 2) {
        printf("Impossible\n");
        return 0;
      }
    }
  }
  printf("Possible\n");
  return 0;
}