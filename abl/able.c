#include <stdio.h>

const long long MOD = 998244353;

long long ten[200010];

int main() {
  int n, q;
  scanf("%d %d", &n, &q);
  ten[n-1] = 1;
  for(int i=n-2;i>=0;i--) ten[i] = ten[i+1] * 10 % MOD;
  
}