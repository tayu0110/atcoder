#include<stdio.h>
#define MAX_N 30
int main() {
  int n;
  scanf("%d", &n);
  int m[MAX_N];
  for(int i = 0; i < n; i++) {
    scanf("%d", m + i);
  }
  int sum = 0;
  for(int i = 0; i < n; i++) {
    sum += (80 - m[i] < 0 ? 0 : 80 - m[i]);
  }
  printf("%d\n", sum);
  return 0;
}