#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int m;
int a[100010];
int kind[100010];
int res[1000010];

struct Query {
  int l;
  int r;
  int index;
} query[1000010];

int comp(const void *a, const void *b) {
  struct Query *l = (struct Query *)a;
  struct Query *r = (struct Query *)b;
  return l->l / m == r->l / m
          ? l->r == r->r ?
                l->index - r->index :
                l->r - r->r
          : l->l / m - r->l / m;
}

int main() {
  int n;
  scanf("%d", &n);
  m = sqrt(n);
  m = m ? m : 1;
  for(int i=0;i<n;i++) scanf("%d", a+i);
  int q;
  scanf("%d", &q);
  for(int i=0;i<q;i++) {
    scanf("%d %d", &(query+i)->l, &(query+i)->r);
    (query+i)->index = i;
  }
  qsort(query, q, sizeof(struct Query), comp);
  int nl = 0, nr = 0;
  int ans = 0;
  for(int i=0;i<q;i++) {
    int l = query[i].l; l--;
    int r = query[i].r;
    int index = query[i].index;
    while(nl > l) {
      nl--; kind[a[nl]]++;
      if(kind[a[nl]] % 2 == 0) ans++;
    }
    while(nr < r) {
      kind[a[nr]]++;
      if(kind[a[nr]] % 2 == 0) ans++;
      nr++;
    }
    while(nl < l) {
      if(kind[a[nl]] % 2 == 0) ans--; 
      kind[a[nl]]--; nl++;
    }
    while(nr > r) {
      nr--;
      if(kind[a[nr]] % 2 == 0) ans--;
      kind[a[nr]]--; 
    }
    res[index] = ans;
  }
  for(int i=0;i<q;i++) printf("%d\n", res[i]);
  return 0;
}