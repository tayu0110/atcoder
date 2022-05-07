#include <stdio.h>

const long long mod = 1000000007;
long long dp[1010];

void swap(char *a, char *b) { char tmp = *a; *a = *b; *b = tmp; }
long long pow_mod(long long a, long long p) {
  if(!p) return 1LL;
  if(p == 1) return a % mod;
  long long res = pow_mod(a, p / 2);
  return p % 2 ? (res * res % mod) * a % mod : res * res % mod;
}

int main() {
  int n;
  char aa, ab, ba, bb;
  scanf("%d\n%c\n%c\n%c\n%c", &n, &aa, &ab, &ba, &bb);
  // printf("%c %c %c %c\n", aa, ab, ba, bb);
  if(n <= 3) {
    printf("1\n");
    return 0;
  }
  if(ab == 'B') {
    swap(&aa, &bb);
    aa ^= 'A' ^ 'B';
    ab ^= 'A' ^ 'B';
    ba ^= 'A' ^ 'B';
    bb ^= 'A' ^ 'B';
  }
  if(aa == 'A') {
    printf("1\n");
    return 0;
  }
  if(ba == 'B') {
    printf("%lld\n", pow_mod(2, n-3));
  } else {
    dp[0] = 1;
    for(int i=2;i<=n;i++) {
      for(int j=0;j<i-1;j++) {
        dp[i] += dp[j];
        dp[i] %= mod;
      }
    }
    printf("%lld\n", dp[n]);
  }
  return 0;
}