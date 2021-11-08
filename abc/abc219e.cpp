#include<cstdio>
#include<cstring>

using namespace std;

int par[17];
void swap(int &l, int &r) {
  int tmp = l; l = r; r = tmp;
}
int root(int x) {
  return (par[x] < 0 ? x : par[x] = root(par[x]));
}
void merge(int x, int y) {
  int rx = root(x);
  int ry = root(y);
  if(rx == ry) return;
  if(par[rx] > par[ry]) swap(rx, ry);
  par[rx] += par[ry];
  par[ry] = rx;
}
int size(int x) {
  return -par[root(x)];
}
#define f(i, j, t) (((i >> j) & 1) == t)
int main(int argc,char* argv[]){
  constexpr int w = 16;
  constexpr int sw = 1 << 16;
  int a = 0, idx;
  for(int i=0;i<w;i++) {
    int tmp;
    scanf("%d", &tmp);
    a |= tmp << i;
    if(tmp) idx = i;
  }
  int ans = 0;
  int up, down, left, right;
  for(int i=0;i<sw;i++) {
    if((a & i) != a) continue;
    memset(par, -1, sizeof(par));
    up = i ^ (i << 4);
    down = i ^ (i >> 4);
    left = i ^ ((i << 1) & 0xEEEE);
    right = i ^ ((i >> 1) & 0x7777);
    for(int j=0;j<w;j++) {
      if(j < 4) {
        if(!(up & 1)) merge(j, w);
        up >>= 1;
      }
      if(!(down & 1)) merge(j, (j > 11 ? w : j+4));
      if(!(j % 4)) {
        if(!(left & 1)) merge(j, w);
        left >>= 4;
      }
      if(!(right & 1)) merge(j, ((j+1) % 4 ? j+1 : w));
      down >>= 1;
      right >>= 1;
    }
    if(size(w) + size(idx) != 17) continue;
    ans++;
  }
  printf("%d\n", ans);
  return 0;
}
