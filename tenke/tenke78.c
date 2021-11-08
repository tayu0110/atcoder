#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;

int main(int argc, char **argv) {
  int n, m;
  scanf("%d %d", &n, &m);
  int t[100010];
  memset(t, 0, sizeof(t));
  for(int i=0;i<m;i++) {
    int a, b;
    scanf("%d %d", &a, &b);
    if(a > b) t[a]++;
    else t[b]++;
  }
  int ans = 0;
  for(int i=0;i<n;i++) if(t[i+1] == 1) ans++;
  printf("%d\n", ans);
  return 0;
}