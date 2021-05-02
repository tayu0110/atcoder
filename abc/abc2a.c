#include<stdio.h>

int main() {
  long long x, y;
  scanf("%lld%lld", &x, &y);
  if(x > y) printf("%lld\n", x);
  else printf("%lld\n", y);
  return 0;
}