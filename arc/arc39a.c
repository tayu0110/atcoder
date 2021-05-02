#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#define LEN 5
void chmax(int *a, int b) {
  if(*a < b) *a = b;
  return;
}
int main() {
  char a[LEN], b[LEN];
  scanf("%s%s", a, b);
  int ans = atoi(a) - atoi(b);
  for(int i=0;i<3;i++) {
    char c[LEN];
    strcpy(c, a);
    c[i] = '9';
    chmax(&ans, atoi(c) - atoi(b));
    // printf("c - b: %d\n", atoi(c) - atoi(b));
    char d[LEN];
    strcpy(d, b);
    if(i == 0) d[i] = '1';
    else d[i] = '0';
    // printf("a - d: %d\n", atoi(a) - atoi(d));
    chmax(&ans, atoi(a) - atoi(d));
  }
  printf("%d\n", ans);
  return 0;
}