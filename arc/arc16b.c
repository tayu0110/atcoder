#include<stdio.h>
#define LEN 105
int main() {
  int n;
  scanf("%d", &n);
  char x[LEN][10];
  int ans = 0;
  for(int i = 0; i < n; i++) {
    scanf("%s", x + i);
    for(int j = 0; j < 9; j++) {
      if(x[i][j] == 'x') ans++;
      if(x[i][j] == 'o') {
        if(i == 0) ans++;
        else {
          if(x[i-1][j] != 'o') ans++;
        }
      }
    }
  }
  printf("%d\n", ans);
  return 0;
}