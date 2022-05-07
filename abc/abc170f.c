#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char **c;
int **map;

#define qsize 1000000
struct Point {
  int h, w;
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

int dx[] = {0, 1, 0, -1};
int dy[] = {1, 0, -1, 0};
int main() {
  int h, w, k;
  scanf("%d %d %d", &h, &w, &k);
  int sx, sy, tx, ty;
  scanf("%d %d %d %d\n", &sx, &sy, &tx, &ty);
  sx--; sy--; tx--; ty--;
  c = (char **)malloc(sizeof(char *) * h);
  map = (int **)malloc(sizeof(int *) * h);
  for(int i=0;i<h;i++) {
    c[i] = (char *)malloc(sizeof(char) * (w+10));
    map[i] = (int *)malloc(sizeof(int) * w);
    memset(map[i], -1, sizeof(int) * w);
    fgets(c[i], w+10, stdin);
    c[i][w] = '\0';
  }
  push((struct Point){ sx, sy });
  map[sx][sy] = 0;
  while(!empty()) {
    struct Point now = pop();
    int x = now.h;
    int y = now.w;
    int nd = map[x][y]+1;
    for(int j=0;j<4;j++) {
      for(int i=1;i<=k;i++) {
        int nx = x + dx[j] * i;
        int ny = y + dy[j] * i;
        if(nx < 0 || nx >= h || ny < 0 || ny >= w) break;
        if(c[nx][ny] == '@') break;
        if(map[nx][ny] != -1 && map[nx][ny] != nd) break;
        if(map[nx][ny] == -1) {
          map[nx][ny] = nd;
          push((struct Point){ nx, ny });
        }
      }
    }
  }
  printf("%d\n", map[tx][ty]);
  return 0;
}