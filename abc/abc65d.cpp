#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
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
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> x(n), y(n);
  for(int i=0;i<n;i++) {
    int s, t;
    cin >> s >> t;
    x[i] = {s, i};
    y[i] = {t, i};
  }
  sort(x.begin(), x.end());
  sort(y.begin(), y.end());
  vector<pair<ll, pii>> p;
  for(int i=0;i<n-1;i++) {
    p.push_back({x[i+1].first-x[i].first, {x[i].second, x[i+1].second}});
    p.push_back({y[i+1].first-y[i].first, {y[i].second, y[i+1].second}});
  }
  sort(p.begin(), p.end());
  UnionFind uf(n);
  ll ans = 0;
  for(int i=0;i<p.size();i++) {
    ll d = p[i].first;
    int s = p[i].second.first, t = p[i].second.second;
    if(uf.isSame(s, t)) continue;
    uf.merge(s, t);
    ans += d;
  }
  cout << ans << endl;
  return 0;
}
