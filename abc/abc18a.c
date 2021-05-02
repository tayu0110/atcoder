#include<stdio.h>

int main() {
  int a, b, c;
  scanf("%d%d%d", &a, &b, &c);
  if(a > b && a > c) {
    if(b > c) printf("1\n2\n3\n");
    else printf("1\n3\n2\n");
  } else if(b > a && b > c) {
    if(c > a) printf("3\n1\n2\n");
    else printf("2\n1\n3\n");
  } else {
    if(a > b) printf("2\n3\n1\n");
    else printf("3\n2\n1\n");
  }
  return 0;
}