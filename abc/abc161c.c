#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;

int main(int argc, char **argv) {
  ll n, k;
  scanf("%lld%lld", &n, &k);
  n %= k;
  ll m = k - n;
  if(n < m) printf("%lld\n", n);
  else printf("%lld\n", m);
  return 0;
}