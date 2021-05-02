#include<stdio.h>
#include<string.h>
#define MAX_N 10
void ps(char *p, int now, int size) {
  if(now == size) {
    char t[MAX_N];
    memset(t, '\0', sizeof(t));
    strncpy(t, p, size);
    printf("%s\n", t);
    return;
  }
  char t[MAX_N];
  memset(t, '\0', sizeof(t));
  for(int i = 0; i < 3; i++) {
    p[now] = 'a' + i;
    ps(p, now+1, size);
  }
  return;
}
int main() {
  int n;
  scanf("%d", &n);
  char p[MAX_N];
  memset(p, '\0', sizeof(p));
  memset(p, 'a', sizeof(char) * n);
  ps(p, 0, n);
  return 0;
}