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
void topo(int now, int par, vector<int> &p, vector<bool> &ck, weightedGraph &t) {
  ck[now] = true;
  for(auto [to, w] : t[now]) {
    if(ck[to]) continue;
    topo(to, now, p, ck, t);
  }
  p.push_back(now);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> p(n);
  for(int i=0;i<n;i++) cin >> p[i], p[i]--;
  int m;
  cin >> m;
  weightedGraph t(n);
  UnionFind uf(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back({b, i});
    t[b].push_back({a, i});
    uf.merge(a, b);
  }
  for(int i=0;i<n;i++) {
    if(!uf.isSame(i, p[i])) {
      cout << -1 << endl;
      return 0;
    }
  }
  vector<int> ans;
  set<int> ck;
  vector<pii> q(n);
  for(int i=0;i<n;i++) q[i] = {p[i], i};
  sort(q.begin(), q.end());
  for(int i=0;i<n;i++) {
    int root = uf.root(i);
    if(ck.find(root) != ck.end()) continue;
    ck.insert(root);
    vector<int> v;
    vector<bool> check(n, false);
    topo(i, -1, v, check, t);
    reverse(v.begin(), v.end());
    for(int j=0;j<v.size();j++) {
      
    }
  }
  return 0;
}
