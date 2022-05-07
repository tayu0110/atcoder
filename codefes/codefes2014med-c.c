#include <stdio.h>
#include <math.h>

int main() {
  long double p;
  long long n;
  scanf("%Lf %lld", &p, &n);
  printf("%.9Lf\n", (1 - powl(1-2*p, n)) / 2);
  return 0;
}