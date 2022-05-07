#include <stdio.h>

long long dp[4][1000010 * 3];

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int main() {
  int n;
  long long k;
  scanf("%d %lld", &n, &k);
  dp[0][0] = 1;
  for(int i=0;i<3;i++) {
    for(int j=0;j<=2*n;j++) {
      dp[i+1][j+1] += dp[i][j];
      dp[i+1][j+n+1] -= dp[i][j];
    }
    for(int j=0;j<n*3;j++) {
      dp[i+1][j+1] += dp[i+1][j];
    }
  }
  long long t = -1;
  for(int i=3;i<=n*3;i++) {
    if(dp[3][i] >= k) {
      t = i;
      break;
    } else k-= dp[3][i];
  }
  int b = 1, d = 1, p = 1;
  while(b <= n) {
    int rem = t - b;
    int dmin = max(1, rem-n);
    int dmax = min(n, rem-1);
    if(dmin > dmax) {
      b++;
      continue;
    }
    if(dmax-dmin+1 < k) {
      k -= dmax - dmin + 1;
      b++;
      continue;
    }
    d = dmin + k - 1;
    p = rem - d;
    printf("%d %d %d\n", b, d, p);
    return 0;
  }
  return 0;
}