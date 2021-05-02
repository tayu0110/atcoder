#include<stdio.h>
#define MAX_M 105
int main() {
  int n, m, a, b;
  scanf("%d%d%d%d", &n, &m, &a, &b);
  int c[MAX_M];
  for(int i = 0; i < m; i++) {
    scanf("%d", c + i);
  }
  for(int i = 0; i < m; i++) {
    if(n <= a) n += b;
    n -= c[i];
    if(n < 0) {
      printf("%d\n", i+1);
      return 0;
    }
  }
  printf("complete\n");
  return 0;
}