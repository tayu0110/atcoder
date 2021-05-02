#include<stdio.h>

int main() {
  int n, m;
  scanf("%d%d", &n, &m);
  if(m - 2*n < 0) {
    printf("-1 -1 -1\n");
    return 0;
  }
  int k = m - 2*n;
  for(int i = 0; k-i >= 0; i++) {
    int b = i;
    if((k - b) % 2 != 0) continue;
    int c = (k - b) / 2;
    if(n - b - c < 0) continue;
    int a = n - b - c;
    printf("%d %d %d\n", a, b, c);
    return 0;
  }
  printf("-1 -1 -1\n");
  return 0;
}