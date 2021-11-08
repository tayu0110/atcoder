#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

typedef long long ll;
typedef long double ld;
int a[110];
int main(int argc, char **argv) {
  int n, m;
  scanf("%d%d", &n, &m);
  for(int i=0;i<n;i++) scanf("%d", a+i);
  int sum = 0;
  for(int i=0;i<n;i++) sum += a[i];
  int cnt = 0;
  for(int i=0;i<n;i++) if(a[i]*m*4 >= sum) cnt++;
  if(cnt >= m) printf("Yes\n");
  else printf("No\n");
  return 0;
}