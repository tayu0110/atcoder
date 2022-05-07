#include <stdio.h>
#include <stdlib.h>

struct Pos {
  int r, c;
  long long score;
} t[1000100];

int comp(const void *a, const void *b) {
  struct Pos *pa = (struct Pos *)a;
  struct Pos *pb = (struct Pos *)b;
  return pa->score == pb->score 
            ? pa->r == pb->r
                ? pa->c - pb->c
                : pa->r - pb->r
            : pa->score - pb->score;
}

int comp2(const void *a, const void *b) {
  struct Pos *pa = (struct Pos *)a;
  struct Pos *pb = (struct Pos *)b;
  return pa->c - pb->c;
}

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int main() {
  int n, m;
  long long x, a, p;
  scanf("%d %d %lld %lld %lld", &n, &m, &x, &a, &p);
  for(int k=0;k<n*m;k++) {
    int r = k / m;
    int c = k % m;
    t[k] = (struct Pos){ r, c, x };
    x += a;
    x %= p;
  }
  qsort(t, n*m, sizeof(struct Pos), comp);
  long long ans = 0;
  for(int i=0;i<n*m;i++) {
    int tr = i / m;
    int nr = t[i].r;
    ans += max(tr, nr) - min(tr, nr);
  }
  int now = 0;
  for(int i=0;i<n;i++) {
    qsort(t+now, m, sizeof(struct Pos), comp2);
    for(int j=0;j<m;j++) {
      int c = t[now+j].c;
      ans += max(c, j) - min(c, j);
    }
    now += m;
  }
  printf("%lld\n", ans);
  return 0;
}