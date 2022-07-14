#include <cstdio>

short a[3000][3000];
char s[3000];

int main() {
  int n, i, j, k;

  scanf("%d", &n);
  for (i = 0; i < n; i++) {
    scanf("%s", s);
    for (j = 0; j < n; j++) {
      a[i][j] = s[j] - '0';
    }
  }

  long ans = 0;
  for (i = 0; i < n; i++) {
    for (j = i+1; j < n; j++) {
      for (k = j+1; k < n; k++) {
        ans += a[i][j] & a[j][k] & a[i][k];
      }
    }
  }

  printf("%ld\n", ans);
}