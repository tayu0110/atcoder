#include<stdio.h>

int main() {
  int n, a, b;
  scanf("%d%d%d", &n, &a, &b);
  int c = b - a - 1;
  if(c % 2 == 0) printf("Borys\n");
  else printf("Alice\n");
  return 0;
}