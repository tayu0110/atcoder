#include<stdio.h>
#define LEN 105
void sort(int *a, int *b, int *c) {
  int tmp;
  if(*a < *b) {
    tmp = *b;
    *b = *a;
    *a = tmp;
  }
  if(*b < *c) {
    tmp = *c;
    *c = *b;
    *b = tmp;
  }
  if(*a < *b) {
    tmp = *b;
    *b = *a;
    *a = tmp;
  }
  return;
}
int max(int a, int b) {
  if(a > b) return a;
  else return b;
}
int main() {
  int c;
  scanf("%d", &c);
  int n[LEN], m[LEN], l[LEN];
  int nx = 0, mx = 0, lx = 0;
  for(int i = 0; i < c; i++) {
    scanf("%d%d%d", n+i, m+i, l+i);
    sort(n+i, m+i, l+i);
    nx = max(nx, n[i]);
    mx = max(mx, m[i]);
    lx = max(lx, l[i]);
  }
  printf("%d\n", nx * mx * lx);
  return 0;
}