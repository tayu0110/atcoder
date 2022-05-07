#pragma GCC optimize("Ofast")
#pragma GCC optimize("unroll-loops")
#pragma GCC target("sse,sse2,sse3,ssse3,sse4,fma,abm,mmx,avx,avx2")
#include <stdio.h>

const int init_k = (1 << 26) - 1;
const long long mod = 998244353;


int t[20];
int cache[] = {
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
};
const int fact[] = {
  1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7, 1 << 8, 1 << 9,
  1 << 10, 1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15, 1 << 16, 1 << 17, 1 << 18, 1 << 19,
  1 << 20, 1 << 21, 1 << 22, 1 << 23, 1 << 24, 1 << 25, 1 << 26, 1 << 27, 1 << 28, 1 << 29
};
const int alpha[] = {
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0,
  1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7, 1 << 8, 1 << 9,
  1 << 10, 1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15, 1 << 16, 1 << 17, 1 << 18, 1 << 19,
  1 << 20, 1 << 21, 1 << 22, 1 << 23, 1 << 24, 1 << 25, 1 << 26, 1 << 27, 1 << 28, 1 << 29
};
 
long long pow_mod(long long a, long long p) {
  if(!p) return 1LL;
  if(p == 1) return a % mod;
  long long res = pow_mod(a, p / 2);
  return p & 1 ? (res * res % mod) * a % mod : res * res % mod;
}
 
int main() {
  int n;
  long long l;
  scanf("%d %lld", &n, &l);
  for(int i=0;i<n;i++) {
    char s[30] = {0};
    scanf("%s", s);
    for(int j=0;j<30;j++) t[i] |= alpha[s[j]];
  }
  long long ans = 0;
  for(int i=1;i<fact[n];i++) {
    int k = init_k;
    for(int j=0;j<20;j++) if(i & fact[j]) k &= t[j];
    int pk = __builtin_popcount(k);
    int pi = __builtin_popcount(i);
    long long res = cache[pk];
    if(res < 0) res = cache[pk] = pow_mod(pk, l);
    if(pi & 1) ans += res;
    else ans -= res;
    ans = (ans + mod) % mod;
  }
  printf("%lld\n", ans);
  return 0;
}