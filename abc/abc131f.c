#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long dsu[100010];

void swap(int *a, int *b) { int tmp = *a; *a = *b; *b = tmp; }
int root(int idx) {
  if(dsu[idx] < 0) return idx;
  return dsu[idx] = root(dsu[idx]);
}
int is_same(int l, int r) {
  return root(l) == root(r);
}
long long size(int idx) {
  return -dsu[root(idx)];
}
void merge(int l, int r) {
  int rl = root(l);
  int rr = root(r);
  if(rl == rr) return;
  if(dsu[rl] > dsu[rr]) {
    swap(&rl, &rr); swap(&l, &r);
  }
  dsu[rl] += dsu[rr];
  dsu[rr] = rl;
}

struct Point {
  int y, x;
} p[100010];
int comp(const void *a, const void *b) {
  struct Point *pa = (struct Point *)a;
  struct Point *pb = (struct Point *)b;
  return pa->y - pb->y;
}

int main() {
  long long n;
  scanf("%lld", &n);
  for(int i=0;i<n;i++) {
    int *y = &p[i].y;
    int *x = &p[i].x;
    scanf("%d %d", x, y);
  }
  qsort(p, n, sizeof(struct Point), comp);
  memset(dsu, -1, sizeof(dsu));
  for(int i=0;i<n-1;i++) if(p[i].y == p[i+1].y) merge(p[i].x, p[i+1].x);
  long long ans = 0;
  for(int i=0;i<n;i++) if(!i || p[i-1].y != p[i].y) ans += size(p[i].x);
  printf("%lld\n", ans - n);
  return 0;
}