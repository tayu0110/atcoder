#include <stdio.h>
#include <stdlib.h>
long long a[100010], b[100010];
long long e[100010], o[100010];
long long max(long long a, long long b) { return a > b ? a : b; }
int comp(const void *a, const void *b) { return *(int *)b - *(int *)a; }
int main() {
  int n;
  scanf("%d", &n);
  for(int i=0;i<n;i++) scanf("%lld", a+i);
  for(int i=0;i<n;i++) scanf("%lld", b+i);
  for(int i=0;i<n;i++) {
    if(i % 2) o[i/2] = b[i] - a[i];
    else e[i/2] = b[i] - a[i];
  }
  int m = n / 2;
  qsort(e, m, sizeof(*e), comp);
  qsort(o, m, sizeof(*o), comp);
  long long sum = 0;
  for(int i=0;i<n;i++) sum += a[i];
  long long ans = sum;
  for(int i=0;i<m;i++) {
    sum += e[i] + o[i];
    ans = max(ans, sum);
  }
  printf("%lld\n", ans);
  return 0;
}