#include<stdio.h>

int main() {
  int n, a, b;
  scanf("%d%d%d", &n, &a, &b);
  int m = n % (a + b);
  if(m > a || m == 0) printf("Bug\n");
  else printf("Ant\n");
  return 0;
}