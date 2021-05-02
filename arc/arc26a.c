#include<stdio.h>

int main() {
  int n, a, b;
  scanf("%d%d%d", &n, &a, &b);
  int c = 5;
  n -= c;
  if(n < 0) {
    c += n;
    n = 0;
  }
  printf("%d\n", n * a + b * c);
  return 0;
}