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
vector<map<int, ll>> memo;
vector<ll> d;
ll dfs(int now, int par, ll dist, weightedGraph &t) {
  ll mx = 0;
  for(auto [to, w] : t[now]) {
    if(to == par) continue;
    memo[now][to] = dfs(to, now, w, t);
    mx = max(mx, memo[now][to]);
  }
  return max(dist + d[now], dist + mx);
}
void dfs2(int now, int par, ll mx, weightedGraph &t) {
  // DEBUG(now);DEBUG(par);DEBUG_EN(mx);
  if(par >= 0) memo[now][par] = mx;
  int mi = par;
  int si = -1;
  ll smx = -1;
  for(auto [f, s] : memo[now]) {
    if(f == par) continue;
    if(mx <= s) {
      si = mi;
      smx = mx;
      mi = f;
      mx = s;
    } else if(smx <= s) {
      smx = s;
      si = f;
    }
  }
  if(d[now] >= mx) {
    si = mi;
    smx = mx;
    mi = -1;
    mx = d[now];
  } else if(d[now] >= smx) {
    si = -1;
    smx = d[now];
  }
  // DEBUG(mi);DEBUG(mx);DEBUG(si);DEBUG_EN(smx);
  for(auto [to, w] : t[now]) {
    if(to == par) continue;
    if(to == mi) dfs2(to, now, smx+w, t);
    else dfs2(to, now, mx+w, t);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<tuple<int, int, ll>> p(n-1);
  for(int i=0;i<n-1;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    p[i] = {a, b, c};
  }
  d.assign(n, 0);
  for(int i=0;i<n;i++) cin >> d[i];
  weightedGraph t(n);
  for(auto [a, b, c] : p) {
    a--;b--;
    t[a].push_back(Edge(b, c));
    t[b].push_back(Edge(a, c));
  }
  memo.resize(n);
  dfs(1, -1, 0, t);
  dfs2(1, -1, d[1], t);
  for(int i=0;i<n;i++) {
    ll res = 0;
    for(auto [f, s] : memo[i]) res = max(res, s);
    cout << res << endl;
  }
  return 0;
}
