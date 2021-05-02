#include<stdio.h>
#define ll long long
int main() {
  ll a, b, k, l;
  scanf("%lld%lld%lld%lld", &a, &b, &k, &l);
  ll c = b * (k/l) + a * (k%l);
  ll nc = b * (k/l + 1);
  if(c > nc) printf("%lld\n", nc);
  else printf("%lld\n", c);
  return 0;
}