#include <stdio.h>
#include <stdlib.h>
int l[100010];
int d[100010], dsum[100010];
int comp(const void *a, const void *b) { return *(int *)a - *(int *)b; }
int bin_search(int target, int size, int *a) {
  int l = 0, r = size;
  while(r - l > 1) {
    int m = (r + l) / 2;
    if(a[m] > target) r = m;
    else l = m;
  }
  return r;
}
int main() {
  int all, n, m;
  scanf("%d%d%d", &all, &n, &m);
  for(int i=0;i<n;i++) scanf("%d", l+i);
  for(int i=1;i<n;i++) d[i] = l[i] - l[i-1] - 1;
  qsort(d+1, n-1, sizeof(*d), comp);
  for(int i=1;i<n;i++) dsum[i] = dsum[i-1] + d[i];
  while(m--) {
    int x, y;
    scanf("%d%d", &x, &y);
    int idx = bin_search(x+y, n, d);
    int res = n + dsum[idx-1] + (n-idx) * (x+y);
    if(l[0]-1 < x) res += l[0] - 1;
    else res += x;
    if(all-l[n-1] < y) res += all - l[n-1];
    else res += y;
    printf("%d\n", res);
  }
  return 0;
}