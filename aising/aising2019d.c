#include <stdio.h>

int n, q;
long long a[100010], b[100010], c[100010];

long long absl(long long x) { return x < 0 ? -x : x; }

int lower_bound(long long val) {
  int l = -1, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    if(a[m] < val) l = m;
    else r = m;
  }
  return r;
}
int upper_bound(long long val) {
  int l = -1, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    if(a[m] <= val) l = m;
    else r = m;
  }
  return r;
}

struct Pair {
  int left, right;
};
struct Pair get_term(long long x, int len) {
  struct Pair res;
  res.left = lower_bound(x-len);
  res.right = upper_bound(x+len) - 1;
  return res;
}

long long get_sum(int l, int r) {
  long long res = b[r];
  if(l) res -= b[l-1];
  return res;
}

int main() {
  scanf("%d %d", &n, &q);
  for(int i=0;i<n;i++) {
    scanf("%lld", a+i);
    if(!i) b[i] = a[i];
    else b[i] = b[i-1] + a[i];
  }
  for(int i=0;i<n;i++) if(n % 2 != i % 2) c[i] = a[i];
  for(int i=1;i<n;i++) c[i] += c[i-1];
  while(q--) {
    long long x;
    scanf("%lld", &x);
    long long l = -1, r = 1001001001;
    while(r-l > 1) {
      long long m = (r+l) / 2;
      struct Pair term = get_term(x, m);
      if(term.left > term.right) {
        l = m;
        continue;
      }
      int len = term.right - term.left + 1;
      if(n-1 <= term.right + len) r = m;
      else l = m;
    }
    struct Pair term = get_term(x, r);
    int left = term.left;
    int right = term.right;
    int t = right - left + 1;
    int tak = n - right - 1;
    int aok = t;
    if(tak != aok) {
      right--; aok--; t--;
    }
    long long res = get_sum(right+1, n-1);
    if(left) res += c[left-1];
    printf("%lld\n", res); 
  }
  return 0;
}