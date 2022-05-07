#include <stdio.h>

long long min(long long l, long long r) { return l < r ? l : r; }

long long x[200010], a[200010];
int main() {
  int n;
  long long L;
  scanf("%d %lld", &n, &L);
  for(int i=0;i<n;i++) scanf("%lld %lld", x+i, a+i);
  long long l = 0, r = 1001001001001001;
  while(r - l > 1) {
    long long t = (l + r) / 2;
    long long now = t;
    long long pos = 0;
    for(int i=0;i<n;i++) {
      if(now < t) {
        now = min(t, now + x[i] - pos);
      }
      now -= a[i];
      if(now < 0) break;
      pos = x[i];
    }
    if(now < 0) l = t;
    else r = t;
  }
  printf("%lld\n", r);
  return 0;
}