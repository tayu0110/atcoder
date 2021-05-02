#include<stdio.h>

int main() {
  int n, m;
  scanf("%d%d", &n, &m);
  n %= 12;
  double l, s;
  l = m*6;
  s = (double)n*30+(double)m*0.5;
  double ans = l-s;
  if(ans < 0) ans = -ans;
  if(ans > 180) ans = 360-ans;
  printf("%.10lf\n", ans);
  return 0;
}