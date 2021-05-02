#include<stdio.h>
#include<string.h>
#define MAX_LEN 105
#define MAX_SIZE 45
int main() {
  int L, R;
  scanf("%d%d", &L, &R);
  int l[MAX_LEN], r[MAX_LEN];
  for(int i = 0; i < L; i++) scanf("%d", l + i);
  for(int i = 0; i < R; i++) scanf("%d", r + i);
  int sz[MAX_SIZE];
  memset(sz, 0, sizeof(sz));
  for(int i = 0; i < L; i++) sz[l[i]]++;
  int ans = 0;
  for(int i = 0; i < R; i++) {
    if(!sz[r[i]]) continue;
    ans++;
    sz[r[i]]--;
  }
  printf("%d\n", ans);
  return 0;
}