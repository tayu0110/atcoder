#include <stdio.h>

int n, q, m;
long long a[300010], res[300010];
long long segtree[1500000];
void update(int index, long long val) {
  index += m;
  while(index) {
    segtree[index] ^= val;
    index >>= 1;
  }
}
long long do_get(int l, int r, int now, int a, int b) {
  if(b <= l || r <= a) return 0;
  if(b-a < 1) return 0;
  if(l <= a && b <= r) return segtree[now];
  long long res = 0;
  res ^= do_get(l, r, now*2, a, (a+b)/2);
  res ^= do_get(l, r, now*2+1, (a+b)/2, b);
  return res;
}
long long get(int l, int r) {
  return do_get(l, r, 1, 0, m);
}

int main() {
  scanf("%d %d", &n, &q);
  m=1;
  while(m < n) m <<= 1;
  for(int i=0;i<n;i++) {
    scanf("%lld", a+i);
    update(i, a[i]);
  }
  int cnt = 0;
  for(int i=0;i<q;i++) {
    int t, x;
    long long y;
    scanf("%d %d %lld", &t, &x, &y);
    x--;
    if(t == 1) {
      update(x, y);
    } else {
      res[cnt++] = get(x, y);
    }
  }
  for(int i=0;i<cnt;i++) printf("%lld\n", res[i]);
  return 0;
}