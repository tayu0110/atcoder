#include<stdio.h>

int main() {
  int a, b;
  scanf("%d%d", &a, &b);
  a *= a;
  b *= b;
  if(a < b) printf("Ant\n");
  else if(a > b) printf("Bug\n");
  else printf("Draw\n");
  return 0;
}