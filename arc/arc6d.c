#include <stdio.h>

short mx;
int h, w;
char c[1010][1010];
short d[1010][1010];
short dx[] = {0, 1, 0, -1, 1, -1, 1, -1};
short dy[] = {1, 0, -1, 0, 1, -1, -1, 1};

#define qsize 30
struct Point {
  int y, x;
};
struct Queue {
  int eq, dq;
  struct Point buf[qsize];
} nt;
void push(struct Point val) {
  nt.buf[nt.eq++] = val;
  if(nt.eq == qsize) nt.eq = 0;
}
struct Point pop() {
  struct Point res = nt.buf[nt.dq++];
  if(nt.dq == qsize) nt.dq = 0;
  return res;
}
int empty() {
  return nt.dq == nt.eq;
}

int max(int a, int b) { return a > b ? a : b; }

void dfs(int y, int x, int now) {
  d[y][x] = now;
  mx = max(mx, now);
  for(int i=0;i<4;i++) {
    int ny = y + dy[i];
    int nx = x + dx[i];
    if(ny < 0 || ny >= h || nx < 0 || nx >= w) continue;
    if(c[ny][nx] == '.') {
      d[ny][nx] = 9;
      continue;
    }
    if(d[ny][nx]) continue;
    dfs(ny, nx, now);
  }
  for(int i=4;i<8;i++) {
    int ny = y + dy[i];
    int nx = x + dx[i];
    if(ny < 0 || ny >= h || nx < 0 || nx >= w) continue;
    if(c[ny][nx] == '.') {
      d[ny][nx] = 9;
      continue;
    }
    if(d[ny][nx]) continue;
    int cx1 = x;
    int cy1 = y + dy[i];
    int cx2 = x + dx[i];
    int cy2 = y;
    if(c[cy1][cx1] == 'o' || c[cy2][cx2] == 'o') continue;
    push((struct Point){ ny, nx });
  }
}

void solve(int y, int x) {
  mx = 0;
  int now = 1;
  dfs(y, x, now);
  while(!empty()) {
    struct Point next = pop();
    y = next.y;
    x = next.x;
    if(d[y][x]) continue;
    now++;
    dfs(y, x, now);
  }
}

int main() {
  scanf("%d %d", &h, &w);
  for(int i=0;i<h;i++) scanf("%s", c[i]);
  int A = 0, B = 0, C = 0;
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(c[i][j] == '.') continue;
      if(d[i][j]) continue;
      solve(i, j);
      if(mx == 3) B++;
      else if(mx == 4) A++;
      else C++;
    }
  }
  printf("%d %d %d\n", A, B, C);
  return 0;
}