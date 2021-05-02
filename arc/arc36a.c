#include<stdio.h>
#define MAX_N 100005
int main() {
  int n, k;
  scanf("%d%d", &n, &k);
  int t[MAX_N];
  for(int i = 0; i < n; i++) {
    scanf("%d", t + i);
  }
  for(int i = 2; i < n; i++) {
    int sum = t[i] + t[i-1] + t[i-2];
    if(sum < k) {
      printf("%d\n", i + 1);
      return 0;
    }
  }
  printf("-1\n");
  return 0;
}