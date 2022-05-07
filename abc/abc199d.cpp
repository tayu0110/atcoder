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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
struct UnionFind {
  vector<int> par;
  UnionFind(int n) : par(vector<int>(n, -1)) {}
  int root(int x) {
    if(par[x] < 0) return x;
    return par[x] = root(par[x]);
  }
  bool merge(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(rx, ry);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  bool isSame(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    return rx == ry;
  }
  int size(int x) {
    return -par[root(x)];
  }
};
vector<bool> reach;
vector<int> lst;
void topo(int now, Graph &t) {
  lst.push_back(now);
  reach[now] = true;
  for(auto e : t[now]) {
    if(reach[e]) continue;
    topo(e, t);
  }
}
vector<int> color;
ll dfs(int idx, Graph &t) {
  if(idx == lst.size()) return 1;
  int now = lst[idx];
  set<int> rem = {0, 1, 2};
  for(auto e : t[now]) {
    if(color[e] < 0) continue;
    rem.erase(color[e]);
  }
  if(rem.empty()) return 0;
  ll res = 0;
  for(auto c : rem) {
    color[now] = c;
    res += dfs(idx+1, t);
  }
  color[now] = -1;
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  UnionFind uf(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
    uf.merge(a, b);
  }
  reach.assign(n, false);
  ll ans = 1;
  set<int> ck;
  for(int i=0;i<n;i++) {
    int root = uf.root(i);
    if(ck.find(root) != ck.end()) continue;
    ck.insert(root);
    lst.resize(0);
    topo(root, t);
    color.assign(n, -1);
    ans *= dfs(0, t);
  }
  cout << ans << endl;
  return 0;
}
