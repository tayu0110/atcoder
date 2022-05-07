#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int used[100010], size[100010];
int *t[100010];
char s[310], c[10];

void append(int idx, int val) {
  if(!size[idx]) {
    size[idx] = 1;
    t[idx] = (int *)malloc(sizeof(int));
  }
  if(used[idx] == size[idx]) {
    size[idx] *= 2;
    int *buf = (int *)realloc(t[idx], sizeof(int) * size[idx]);
    t[idx] = buf;
  }
  t[idx][used[idx]++] = val;
}

// #define QSIZE 100010
// struct Queue {
//   int dq, eq;
//   int buf[QSIZE];
// } nt;
// void push(int val) {
//   nt.buf[nt.eq++] = val;
//   if(nt.eq == QSIZE) nt.eq = 0;
// }
// int pop() {
//   int res = nt.buf[nt.dq++];
//   if(nt.dq == QSIZE) nt.dq = 0;
//   return res;
// }
// int qsize() {
//   if(nt.eq >= nt.dq) return nt.eq - nt.dq;
//   else return nt.eq+1 + QSIZE-nt.dq;
// }

int n, m;
int ck[100010], prev[100010];
void solve(int l, int r) {
  int w = 0;
  if(s[l] == '\"') {
    l++;
    if(s[r] == '\"') r--;
    else {
      w = 1;
      r -= 3;
    }
    solve(l, r);
    memcpy(prev, ck, sizeof(prev));
    memset(ck, 0, sizeof(ck));
    int sz = 0;
    for(int i=0;i<n;i++) sz += prev[i];
    if(w) {
      for(int i=0;i<n;i++) for(int j=0;j<used[i];j++) if(prev[t[i][j]]) ck[i] = 1;
    } else {
      for(int i=0;i<n;i++) {
        int k = 0;
        for(int j=0;j<used[i];j++) if(prev[t[i][j]]) k++;
        if(k < sz) ck[i] = 1;
      }
    }
  } else {
    int idx = 0;
    memset(c, 0, sizeof(c));
    memset(ck, 0, sizeof(ck));
    l += 5;
    if(s[r] == 'w') w = 1;
    while('0' <= s[l] && s[l] <= '9') c[idx++] = s[l++];
    int k = atoi(c) - 1;
    if(w) {
      for(int i=0;i<n;i++) for(int j=0;j<used[i];j++) if(t[i][j] == k) ck[i] = 1;
    } else {
      for(int i=0;i<n;i++) ck[i] = 1;
      for(int i=0;i<n;i++) for(int j=0;j<used[i];j++) if(t[i][j] == k) ck[i] = 0;
    }
  }
}

int main() {
  scanf("%d %d", &n, &m);
  for(int i=0;i<m;i++) {
    int a, b;
    scanf("%d %d", &a, &b);
    a--; b--;
    append(a, b);
  }
  scanf("%s", s);
  int len = strlen(s);
  solve(0, len-1);
  int ans = 0;
  for(int i=0;i<n;i++) ans += ck[i];
  printf("%d\n", ans);
  return 0;
}