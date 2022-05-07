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
const ld PI = acos(-1);

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};
using weightedGraph = vector<vector<Edge>>;

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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, q;
  cin >> n >> m >> q;
  vector<tuple<int, int, ll>> t(m);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;b--;
    t[i] = {a, b, c};
  }
  vector<tuple<int, int, int, ll>> p(q);
  for(int i=0;i<q;i++) {
    int u, v;
    ll w;
    cin >> u >> v >> w;
    u--;v--;
    p[i] = {i, u, v, w};
  }
  vector<tuple<ll, int, int, int>> v(m+q);
  for(int i=0;i<m;i++) {
    auto [a, b, c] = t[i];
    v[i] = {c, a, b, -1};
  }
  for(int i=0;i<q;i++) {
    auto [idx, a, b, c] = p[i];
    v[i+m] = {c, a, b, idx};
  }
  sort(v.begin(), v.end());
  UnionFind uf(n);
  vector<int> ans(q);
  for(int i=0;i<m+q;i++) {
    auto [c, a, b, idx] = v[i];
    if(uf.isSame(a, b)) {
      if(idx >= 0) ans[idx] = 0;
      continue;
    }
    if(idx >= 0) {
      ans[idx] = 1;
      continue;
    }
    uf.merge(a, b);
  }
  for(auto e : ans) {
    if(e) cout << "Yes" << endl;
    else cout << "No" << endl;
  }
  return 0;
}
