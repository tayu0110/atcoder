#include<stdio.h>
#define MAX_N 105
int main() {
  int n;
  scanf("%d", &n);
  char s[MAX_N][MAX_N];
  for(int i = 0; i < n; i++) {
    scanf("%s", s + i);
  }
  int a = 0;
  int t = 0;
  for(int i = 0; i < n; i++) {
    for(int j = 0; j < n; j++) {
      if(s[i][j] == 'R') t++;
      else if(s[i][j] == 'B') a++;
      else continue;
    }
  }
  if(t > a) printf("TAKAHASHI\n");
  else if(t < a) printf("AOKI\n");
  else printf("DRAW\n");
  return 0;
}