#include <stdio.h>
#include <stdlib.h>

int t[10];
long long w[10], l[100010], v[100010];
long long dp[10][10];
struct Parts {
  long long len;
  long long w;
} p[100010];

long long max(long long a, long long b) { return a > b ? a : b; }
long long min(long long a, long long b) { return a < b ? a : b; }
int comp(const void *a, const void *b) {
  struct Parts *pa = (struct Parts *)a;
  struct Parts *pb = (struct Parts *)b;
  return pa->w == pb->w ? pb->len - pa->len : pa->w - pb->w;
}
int comp_int(const void *a, const void *b) {
  int *pa = (int *)a;
  int *pb = (int *)b;
  return *pa - *pb;
}
void swap(int *a, int *b) { int tmp = *a; *a = *b; *b = tmp; }
void reverse(int *w, size_t n) {
  int l = 0, r = n-1;
  while(l < r) swap(&w[l++], &w[r--]);
}
int next_permutation(int *w, size_t n) {
  for(int i=n-2;i>=0;i--) {
    if(w[i] < w[i+1]) {
      int j = n;
      do {
        j--;
      } while(w[i] >= w[j]);
      swap(&w[i], &w[j]);
      qsort(w+i+1, n-i-1, sizeof(int), comp_int);
      return 1;
    }
    if(i == 0) {
      reverse(w, n);
    }
  }
  return 0;
}

int main() {
  int n, m;
  scanf("%d %d", &n, &m);
  for(int i=0;i<n;i++) scanf("%lld", w+i);
  int f = 0;
  for(int i=0;i<m;i++) {
    scanf("%lld %lld", l+i, v+i);
    for(int j=0;j<n;j++) f |= w[j] > v[i];
    p[i] = (struct Parts){ l[i], v[i] };
  }
  p[m] = (struct Parts){ 0, 0 };
  m++;
  if(f) {
    printf("-1\n");
    return 0;
  }
  qsort(p, m, sizeof(struct Parts), comp);
  for(int i=1;i<m;i++) {
    p[i].len = max(p[i].len, p[i-1].len);
  }
  for(int i=0;i<n;i++) t[i] = i;
  long long ans = 1LL << 60;
  do {
    for(int i=0;i<n;i++) for(int j=0;j<n;j++) dp[i][j] = 1LL << 60;
    for(int i=0;i<n;i++) {
      long long w_sum = w[t[i]];
      for(int j=i+1;j<n;j++) {
        w_sum += w[t[j]];
        int l = 0, r = m;
        while(r-l > 1) {
          int mid = (r+l) / 2;
          if(p[mid].w < w_sum) l = mid;
          else r = mid;
        }
        dp[i][j] = -p[l].len;
      }
    }
    for(int k=0;k<n;k++) for(int i=0;i<n;i++) for(int j=0;j<n;j++) dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
    ans = min(ans, -dp[0][n-1]);
  } while(next_permutation(t, n));
  printf("%lld\n", ans);
  return 0;
}