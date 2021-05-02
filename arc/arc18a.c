#include<stdio.h>

int main() {
  double h, b;
  scanf("%lf%lf", &h, &b);
  double w = b * h * h / 100 / 100;
  printf("%.10lf\n", w);
  return 0;
}