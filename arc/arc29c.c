#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int dsu[100010];

struct Edge {
  long long w;
  int a, b;
} p[400010];

void swap(int *a, int *b) { int tmp = *a; *a = *b; *b = tmp; }

int root(int idx) {
  if(dsu[idx] < 0) return idx;
  return dsu[idx] = root(dsu[idx]);
}
int is_same(int l, int r) {
  return root(l) == root(r);
}
void merge(int l, int r) {
  int rl = root(l);
  int rr = root(r);
  if(dsu[rl] > dsu[rr]) swap(&rl, &rr);
  dsu[rl] += dsu[rr];
  dsu[rr] = rl;
}

int comp(const void *a, const void *b) {
  struct Edge *pa = (struct Edge *)a;
  struct Edge *pb = (struct Edge *)b;
  return pa->w == pb->w
            ? pa->a == pb->a
                  ? pa->b - pb->b
                  : pa->a - pb->a
            : pa->w - pb->w;
}

int main() {
  int n, m;
  scanf("%d %d", &n, &m);
  for(int i=0;i<n;i++) {
    long long c;
    scanf("%lld", &c);
    p[i] = (struct Edge){ c, i, n };
  }
  for(int i=0;i<m;i++) {
    int a, b;
    long long r;
    scanf("%d %d %lld", &a, &b, &r);
    a--; b--;
    p[i+n] = (struct Edge){ r, a, b };
  }
  memset(dsu, -1, sizeof(dsu));
  qsort(p, n+m, sizeof(struct Edge), comp);
  long long ans = 0;
  for(int i=0;i<n+m;i++) {
    struct Edge *now = &p[i];
    int a = now->a;
    int b = now->b;
    long long w = now->w;
    if(is_same(a, b)) continue;
    ans += w;
    merge(a, b);
  }
  printf("%lld\n", ans);
  return 0;
}