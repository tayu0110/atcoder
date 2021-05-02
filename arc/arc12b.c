#include<stdio.h>

int main() {
  int n;
  double va, vb, l;
  scanf("%d%lf%lf%lf", &n, &va, &vb, &l);
  double t = 0;
  while(n--) {
    double tmp = l;
    l += (l - t) * vb / va;
    t = tmp;
  }
  printf("%.10lf\n", l - t);
  return 0;
}