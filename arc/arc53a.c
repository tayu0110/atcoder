#include<stdio.h>

int main() {
  int h, w;
  scanf("%d%d", &h, &w);
  printf("%d\n", (1 + (w - 2)) * h + (1 + (h - 2)) * w);
  return 0;
}