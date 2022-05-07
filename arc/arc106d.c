#include <stdio.h>

const long long mod = 998244353;

long long a[200010], fact[310], ifact[310];
long long pa[310][200010], pa_sum[310][200010];

long long pow_mod(long long a, long long p) {
  if(!p) return 1LL;
  if(p == 1) return a % mod;
  long long res = pow_mod(a, p / 2);
  return p % 2 ? (res * res % mod) * a % mod : res * res % mod;
}
long long get_inv(long long a) {
  return pow_mod(a, mod-2);
}
void init_comb() {
  fact[0] = ifact[0] = 1;
  for(long long i=1;i<310;i++) {
    fact[i] = fact[i-1] * i % mod;
    ifact[i] = ifact[i-1] * get_inv(i) % mod;
  }
}
long long c(long long n, long long k) {
  return (fact[n] * ifact[n-k] % mod) * ifact[k] % mod;
}

int main() {
  int n, k;
  scanf("%d %d", &n, &k);
  for(int i=0;i<n;i++) {
    scanf("%lld", a+i);
    pa[0][i] = 1;
    for(int j=1;j<=k;j++) pa[j][i] = pa[j-1][i] * a[i] % mod;
  }
  for(int i=0;i<=k;i++) {
    pa_sum[i][0] = pa[i][0];
    for(int j=1;j<n;j++) pa_sum[i][j] = (pa_sum[i][j-1] + pa[i][j]) % mod;
  }
  init_comb();
  for(int x=1;x<=k;x++) {
    long long res = 0;
    for(int i=0;i<=x;i++) {
      res += (c(x, i) * pa_sum[x-i][n-1] % mod) * pa_sum[i][n-1] % mod;
      res %= mod;
    }
    for(int i=0;i<n;i++) {
      res -= pow_mod(2*a[i], x);
      res += mod;
      res %= mod;
    }
    res *= get_inv(2);
    res %= mod;
    // for(int j=0;j<n-1;j++) {
    //   for(int i=0;i<=x;i++) {
    //     // fprintf(stderr, "j: %d, i: %d, c: %lld, pa: %lld, pa_sum[n-1]-pa_sum[j]: %lld\n", j, i, c(x, i), pa[x-i][j], (pa_sum[i][n-1]-pa_sum[i][j]+mod)%mod);
    //     res += (c(x, i) * pa[x-i][j] % mod) * ((pa_sum[i][n-1] - pa_sum[i][j] + mod) % mod) % mod;
    //     res %= mod;
    //   }
    // }
    printf("%lld\n", res);
  }
}