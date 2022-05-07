#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const double PI = acos(-1);

int dx[] = {1, 0, -1, 0};
int dy[] = {0, 1, 0, -1};
char c[510][510];
int ck[250010];
double cache[250010];
#define f(y, x, m) ( y*m + x )
#define rf(id, m) make_pair(id / m, id % m) 

double mpow(double val, int p) {
  if(!p) return cache[p] = 1;
  if(p == 1) return cache[p] = val;
  if(cache[p] > -0.5) return cache[p];
  double res = mpow(val, p/2);
  return p & 1 ? res * res * val : res * res;
}

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, m;
  scanf("%d %d", &n, &m);
  int s = -1, g = -1;
  for(int i=0;i<n;i++) {
    scanf("%s", c[i]);
    for(int j=0;j<m;j++) {
      if(c[i][j] == 's') s = f(i, j, m);
      if(c[i][j] == 'g') g = f(i, j, m);
    }
  }
  double l = -1, r = 10;
  for(int i=0;i<n*m+1;i++) cache[i] = -1;
  while(r-l > 1e-10) {
    double mn = (r+l) / 2;
    queue<pii> nt;
    nt.push({s, 0});
    memset(ck, -1, sizeof(int)*n*m);
    while(!nt.empty()) {
      auto [id, nd] = nt.front();
      nt.pop();
      if(ck[id] >= 0) continue;
      ck[id] = nd;
      auto [y, x] = rf(id, m);
      for(int i=0;i<4;i++) {
        int nx = x + dx[i];
        int ny = y + dy[i];
        if(nx < 0 || nx >= m || ny < 0 || ny >= n) continue;
        if(c[ny][nx] == '#') continue;
        int nid = f(ny, nx, m);
        if(c[ny][nx] == 'g') {
          ck[nid] = nd+1;
          break;
        }
        if(ck[nid] >= 0) continue;
        if(cache[nd+1] < 0) cache[nd+1] = mpow(0.99, nd+1);
        double now = cache[nd+1] * (c[ny][nx] - '0');
        if(now < mn - 1e-12) continue;
        nt.push({nid, nd+1});
      }
      if(ck[g] >= 0) break;
    }
    if(ck[g] >= 0) l = mn;
    else r = mn;
  }
  if(r < 0) cout << -1 << endl;
  else cout << r << endl;
  return 0;
}
