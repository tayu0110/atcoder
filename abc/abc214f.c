#include <stdio.h>
#include <string.h>

const int mod = 1000000007;
char s[200010];
int next[200010][26];
long long dp[200010];

int main() {
  scanf("%s", s);
  int n = strlen(s);
  for(int j=0;j<26;j++) next[n][j] = -1;
  for(int i=n;i>0;i--) {
    for(int j=0;j<26;j++) next[i-1][j] = next[i][j];
    next[i-1][s[i-1]-'a'] = i;
  }
  memset(dp, 0, sizeof(dp));
  dp[0] = 1;
  for(int i=0;i<n;i++) {
    for(int j=0;j<26;j++) {
      if(next[i][j] < 0) continue;
      if(i && next[i][j]+1 == i+1) continue;
      dp[next[i][j]+1] += dp[i];
      dp[next[i][j]+1] %= mod;
    }
  }
  long long ans = 0;
  for(int i=1;i<n+2;i++) {
    ans += dp[i];
    ans %= mod;
  }
  printf("%lld\n", ans);
  return 0;
}