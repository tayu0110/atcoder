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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
struct UnionFind {
  vector<int> par;
  UnionFind(int n) : par(vector<int>(n, -1)) {}
  int root(int x) { return par[x] < 0 ? x : par[x] = root(par[x]); }
  bool isSame(int x, int y) { return root(x) == root(y); }
  int size(int x) { return -par[root(x)]; }
  bool merge(int x, int y) {
    int rx = root(x), ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(rx, ry);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  void clear() {
    for(int i=0;i<par.size();i++) par[i] = -1;
  }
};
vector<pii> t1, t2;
void dfs(int now, vector<vector<int>> &t, UnionFind &uf) {
  for(auto to : t[now]) {
    if(uf.isSame(now, to)) continue;
    uf.merge(now, to);
    t1.push_back({now, to});
    dfs(to, t, uf);
  }
}
int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<vector<int>> t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--; v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  UnionFind uf(n);
  dfs(0, t, uf);
  uf.clear();
  queue<int> nt;
  nt.push(0);
  while(!nt.empty()) {
    int now = nt.front(); nt.pop();
    for(auto to : t[now]) {
      if(uf.isSame(now, to)) continue;
      uf.merge(now, to);
      t2.push_back({now, to});
      nt.push(to);
    }
  }
  for(auto [f, s] : t1) cout << f+1 << " " << s+1 << endl;
  for(auto [f, s] : t2) cout << f+1 << " " << s+1 << endl;
  return 0;
}
