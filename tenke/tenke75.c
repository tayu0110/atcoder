#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <stdarg.h>

typedef long long ll;
typedef long double ld;

int main(int argc, char **argv) {
  ll n;
  scanf("%lld", &n);
  int res = 0;
  ll tmp = n;
  for(ll i=2;i*i<=n;i++) {
    while(tmp % i == 0) {
      res++;
      tmp /= i;
    }
  }
  if(tmp != 1) res++;
  int now = 1;
  int ans = 0;
  while(now < res) {
    ans++;
    now *= 2;
  }
  printf("%d\n", ans);
  return 0;
}