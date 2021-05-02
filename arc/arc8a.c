#include<stdio.h>

int main() {
  int n;
  scanf("%d", &n);
  int ans = 50000;
  for(int i = 0; i < 6; i++) {
    for(int j = 0; j < 51; j++) {
      if(i * 10 + j < n) continue;
      if(i * 100 + j * 15 < ans) ans = i*100 + j*15;
    }
  }
  printf("%d\n", ans);
  return 0;
}