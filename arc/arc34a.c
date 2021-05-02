#include<stdio.h>
#define MAX_N 3050
int main() {
  int n;
  scanf("%d", &n);
  double a[MAX_N], b[MAX_N], c[MAX_N], d[MAX_N], e[MAX_N];
  double p[MAX_N];
  double ans = 0;
  for(int i = 0; i < n; i++) {
    scanf("%lf%lf%lf%lf%lf", a + i, b + i, c + i, d + i, e + i);
    p[i] = a[i] + b[i] + c[i] + d[i] + e[i] * 110 / 900;
    ans = (ans < p[i] ? p[i] : ans);
  }
  printf("%.10lf\n", ans);
  return 0;
}