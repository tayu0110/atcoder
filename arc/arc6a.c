#include<stdio.h>
#include<stdbool.h>
int main() {
  int e[6];
  for(int i = 0; i < 6; i++) scanf("%d", e + i);
  int b;
  scanf("%d", &b);
  int l[6];
  for(int i = 0; i < 6; i++) scanf("%d", l + i);
  int m = 0;
  bool s = false;
  for(int i = 0; i < 6; i++) {
    bool flag = false;
    for(int j = 0; j < 6; j++) {
      if(l[i] == e[j]) {
        m++;
        flag = true;
      }
    }
    if(!flag) {
      if(l[i] == b) s = true;
    }
  }
  int ans;
  if(m == 6) ans = 1;
  else if(m == 5 && s) ans = 2;
  else if(m == 5) ans = 3;
  else if(m == 4) ans = 4;
  else if(m == 3) ans = 5;
  else ans = 0;
  printf("%d\n", ans);
  return 0;
}