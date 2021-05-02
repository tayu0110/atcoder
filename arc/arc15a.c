#include<stdio.h>

int main() {
  double n;
  scanf("%lf", &n);
  double ans = (1.8 * n) + 32;
  printf("%.10lf\n", ans);
  return 0;
}