#include<stdio.h>
#define MAX_N 105
void sort(int l, int r, int *a) {
  if(r-l <= 1) return;
  int m = (l+r)/2;
  int lhs = l, rhs = r-1;
  while(lhs < rhs) {
    if(a[lhs] >= a[m] && a[rhs] <= a[m]) {
      int tmp = a[lhs];
      a[lhs] = a[rhs];
      a[rhs] = tmp;
      if(lhs == m) {
        m = rhs;
        lhs++;
      } else if(rhs == m) {
        m = lhs; 
        rhs--;
      } else {
        lhs++;
        rhs--;
      }
    } else if(a[lhs] >= a[m]) {
      rhs--;
    } else if(a[rhs] <= a[m]) {
      lhs++;
    } else {
      lhs++;
      rhs--;
    }
  }
  sort(l, m, a);
  sort(m+1, r, a);
  return;
}
int main() {
  int n;
  scanf("%d", &n);
  int a[MAX_N];
  for(int i = 0; i < n; i++) {
    scanf("%d", a+i);
  }
  sort(0, n, a);
  int ans = a[n-1];
  for(int i = n-2; i >= 0; i--) {
    if(a[i] == ans) continue;
    ans = a[i];
    break;
  }
  printf("%d\n", ans);
  return 0;
}