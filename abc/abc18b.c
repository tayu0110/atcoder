#include<stdio.h>
#define MAX_LEN 105
int main() {
  char s[MAX_LEN];
  int n;
  scanf("%s", s);
  scanf("%d", &n);
  for(int i = 0; i < n; i++) {
    int l, r;
    scanf("%d%d", &l, &r);
    l--;r--;
    for(int j = l; j < r+1; j++) {
      char a = s[j];
      char b = s[r];
      s[j] = b;
      s[r] = a;
      r--;
    }
  }
  printf("%s\n", s);
  return 0;
}