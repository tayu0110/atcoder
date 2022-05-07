#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct Node {
  int number, index;
};
struct Node *t[55];
int sz[55], used[55];
long long path[55];

void append(int idx, struct Node val) {
  if(!sz[idx]) {
    t[idx] = (struct Node *)malloc(sizeof(struct Node));
    sz[idx]++;
  }
  if(used[idx] == sz[idx]) {
    sz[idx] *= 2;
    struct Node *buf = (struct Node *)realloc(t[idx], sizeof(struct Node) * sz[idx]);
    t[idx] = buf;
  }
  t[idx][used[idx]++] = val;
}

long long dfs(int now, int par, int target) {
  for(int i=0;i<used[now];i++) {
    struct Node next = t[now][i];
    int to = next.number;
    int index = next.index;
    if(to == par) continue;
    if(to == target) {
      return 1LL << index;
    }
    long long res = dfs(to, now, target);
    if(res) return res | (1LL << index);
  }
  return 0;
}

int popcount(long long val) {
  int res = 0;
  while(val) {
    res += val & 1;
    val >>= 1;
  }
  return res;
}

int main() {
  int n;
  scanf("%d", &n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    scanf("%d %d", &a, &b);
    a--; b--;
    append(a, (struct Node){ b, i });
    append(b, (struct Node){ a, i });
  }
  int m;
  scanf("%d", &m);
  for(int i=0;i<m;i++) {
    int u, v;
    scanf("%d %d", &u, &v);
    u--; v--;
    path[i] = dfs(u, -1, v);
  }
  long long ans = 0;
  for(int i=0;i<(1<<m);i++) {
    long long t = 0;
    for(int j=0;j<m;j++) if(i & (1<<j)) t |= path[j];
    int pc = popcount(t);
    long long k = 1LL << (n-1 - pc);
    if(__builtin_popcount(i) % 2) ans -= k;
    else ans += k;
  }
  printf("%lld\n", ans);
  return 0;
}