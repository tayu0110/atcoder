#include<stdio.h>

int main() {
  int n;
  scanf("%d", &n);
  if(n < 10) {
    printf("%d\n", n);
    return 0;
  }
  int i = 10;
  n -= 9;
  while(n) {
    i++;
    if(i >= 1000000) {
      if(i % 1111111 == 0) n--;
    } else if(i >= 100000) {
      if(i % 111111 == 0) n--;
    } else if(i >= 10000) {
      if(i % 11111 == 0) n--;
    } else if(i >= 1000) {
      if(i % 1111 == 0) n--;
    } else if(i >= 100) {
      if(i % 111 == 0) n--;
    } else {
      if(i % 11 == 0) n--;
    }
  }
  printf("%d\n", i);
  return 0;
}