#include <stdio.h>

int main() {
  int n;
  scanf("%d", &n);
  if(n == 1) {
    printf("1 1\n");
    return 0;
  }
  for(int i=1;i<=n;i++) {
    int l = 2*i;
    int r = 2*i+1;
    l %= n; if(!l) l = n;
    r %= n; if(!r) r = n;
    if(r <= n) {
      printf("%d %d\n", l, r);
    } else {
      if(l <= n) {
        printf("%d %d\n", l, 1);
      } else {
        printf("%d %d\n", 1, 2);
      }
    }
  }
  return 0;
}