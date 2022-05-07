#include <stdio.h>
#include <stdlib.h>

int n, m;
int segtree[1000010];
long long b[200010], c[200010];

void update(int idx, int val) {
  idx += m;
  while(idx) {
    segtree[idx] += val;
    idx /= 2;
  }
}
int do_get_sum(int l, int r, int now, int a, int b) {
  if(b <= l || r <= a) return 0;
  if(l <= a && b <= r) return segtree[now];
  int res = 0;
  res += do_get_sum(l, r, 2*now, a, (a+b) / 2);
  res += do_get_sum(l, r, 2*now+1, (a+b) / 2, b);
  return res;
}
int get_sum(int l, int r) {
  return do_get_sum(l, r, 1, 0, m);
}

int comp(const void *a, const void *b) { return *(long long *)a - *(long long *)b; }

int lower_bound(long long val) {
  int l = 0, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    if(c[m] < val) l = m;
    else r = m;
  }
  return r;
}
int upper_bound(long long val) {
  int l = 0, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    if(c[m] <= val) l = m;
    else r = m;
  }
  return r;
}

int main() {
  long long k;
  scanf("%d %lld", &n, &k);
  for(int i=0;i<n;i++) {
    long long a;
    scanf("%lld", &a);
    a -= k;
    b[i+1] = a + b[i];
    c[i+1] = b[i+1];
  }
  n++;
  qsort(c, n, sizeof(long long), comp);
  m = 1;
  while(m < n) m <<= 1;
  long long ans = 0;
  for(int i=0;i<n;i++) {
    int idx = upper_bound(b[i]);
    ans += get_sum(0, idx);
    update(lower_bound(b[i]), 1);
  }
  printf("%lld\n", ans);
  return 0;
}