#include<stdio.h>
#include<string.h>
#define MAX_LEN 55
int main() {
  char a[MAX_LEN], b[MAX_LEN];
  scanf("%s%s", a, b);
  int la, lb;
  la = strlen(a), lb = strlen(b);
  if(la < lb) printf("%s\n", b);
  else printf("%s\n", a);
  return 0;
}