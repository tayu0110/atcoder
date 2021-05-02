#include<stdio.h>

int main() {
  int n, m, l, p, q, r;
  scanf("%d%d%d%d%d%d", &n, &m, &l, &p, &q, &r);
  int ans = (n/p) * (m/q) * (l/r);
  int d = (n/p) * (m/r) * (l/q);
  if(ans < d) ans = d;

  d = (n/q) * (m/p) * (l/r);
  if(ans < d) ans = d;
  
  d = (n/q) * (m/r) * (l/p);
  if(ans < d) ans = d;
  
  d = (n/r) * (m/p) * (l/q);
  if(ans < d) ans = d;
  
  d = (n/r) * (m/q) * (l/p);
  if(ans < d) ans = d;
  printf("%d\n", ans);
  return 0;
}