#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int dist[1010][1010];
struct Node {
  int index;
  char c;
};
struct Map {
  int size;
  struct Node next[1010];
} map[1010];
void append(int index, int next, char c) {
  map[index].next[map[index].size++] = (struct Node){next, c};
}

struct Pair {
  int l, r, dist;
};
#define qsize 1000000
struct Queue {
  int dq;
  int eq;
  struct Pair buf[qsize];
} nt;
void push(struct Pair pair) {
  nt.buf[nt.eq++] = pair;
  if(nt.eq == qsize) nt.eq = 0;
}
struct Pair pop(void) {
  struct Pair res = nt.buf[nt.dq++];
  if(nt.dq == qsize) nt.dq = 0;
  return res;
}
int empty(void) {
  return nt.dq == nt.eq;
}

int min(int a, int b) { return a < b ? a : b; }

int main() {
  int n, m;
  scanf("%d %d", &n, &m);
  memset(map, 0, sizeof(map));
  for(int i=0;i<m;i++) {
    int a, b;
    char c;
    scanf("%d %d %c", &a, &b, &c);
    append(a, b, c);
    append(b, a, c);
  }
  push((struct Pair){1, n, 0});
  int ans = 100100100;
  memset(dist, -1, sizeof(dist));
  while(!empty()) {
    struct Pair now = pop();
    int l = now.l;
    int r = now.r;
    int nd = now.dist;
    if(dist[l][r] >= 0) continue;
    dist[l][r] = nd;
    if(l == r) {
      ans = min(ans, nd*2);
      continue;
    }
    struct Node *l_next = map[l].next;
    struct Node *r_next = map[r].next;
    for(int i=0;i<map[l].size;i++) {
      int to_l = l_next[i].index;
      char c_l = l_next[i].c;
      if(to_l == r) {
        ans = min(ans, nd*2+1);
        break;
      }
      for(int j=0;j<map[r].size;j++) {
        int to_r = r_next[j].index;
        char c_r = r_next[j].c;
        if(c_l == c_r && dist[to_l][to_r] < 0) push((struct Pair){to_l, to_r, nd+1});
      }
    }
  }
  printf("%d\n", ans == 100100100 ? -1 : ans);
  return 0;
}