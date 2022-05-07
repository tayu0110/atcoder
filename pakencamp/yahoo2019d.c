#include <stdio.h>

long long a[200010];
long long dp[200010][5];

long long min(long long a, long long b) { return a < b ? a : b; }

int main() {
  int l;
  scanf("%d", &l);
  for(int i=0;i<l;i++) scanf("%lld", a+i);
  for(int i=1;i<5;i++) dp[0][i] = 1LL << 60;
  for(int i=0;i<l;i++) {
    for(int j=0;j<5;j++) {
      long long mn = 1LL << 60;
      for(int k=0;k<=j;k++) mn = min(mn, dp[i][k]);
      dp[i+1][j] = mn;
      if(j == 0 || j == 4) dp[i+1][j] += a[i];
      else if(j == 1 || j == 3) dp[i+1][j] += a[i] == 0 ? 2 : a[i] % 2;
      else dp[i+1][j] += 1 - a[i] % 2;
    }
  }
  long long ans = 1LL << 60;
  for(int i=0;i<5;i++) ans = min(ans, dp[l][i]);
  printf("%lld\n", ans);
  return 0;
}