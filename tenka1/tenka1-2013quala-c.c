#include <stdio.h>

int n, m;
short t[100][100];

long long solve(int y, int x) {
  long long res = 0;
  for(int i=1;i<=3;i++) {
    int flag = -1;
    for(int j=1;j<=i;j++) {
      if(y-j >= 0 && t[y-j][x] == i) flag = 0;
      if(x-j >= 0 && t[y][x-j] == i) flag = 0;
      if(!flag) break;
    }
    if(!flag) continue;
    t[y][x] = i;
    if(x == m-1 && y == n-1) {
      // fprintf(stderr, "<====== ======>\n");
      // for(int i=0;i<n;i++) {
      //   for(int j=0;j<m;j++) fprintf(stderr, "%d ", t[i][j]);
      //   fprintf(stderr, "\n");
      // }
      res++;
    }
    else if(x == m-1) res += solve(y+1, 0);
    else res += solve(y, x+1);
  }
  return res;
}

int main() {
  scanf("%d %d", &n, &m);
  if(n < 100 && m < 100) {
    printf("%lld\n", solve(0, 0));
  } else if(n == 1 || m == 1) {
    int k = n == 1 ? m % 4 : n % 4;
    if(k == 0) printf("10\n");
    else if(k == 1 || k == 3) printf("9\n");
    else printf("8\n");
  } else {
    int k = (n+m) % 4;
    if(k == 1) printf("20\n");
    else if(k == 0 || k == 2) printf("18\n");
    else printf("16\n");
  }
  return 0;
}