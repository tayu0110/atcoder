#include<stdio.h>

int main() {
  long long a, b;
  scanf("%lld%lld", &a, &b);
  if(a > 0 && b > 0 || a < 0 && b < 0) printf("%lld\n", b - a);
  else printf("%lld\n", b - a - 1);
  return 0;
}