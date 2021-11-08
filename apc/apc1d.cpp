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
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

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
  int n, m;
  cin >> n >> m;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<pll> p(n);
  for(int i=0;i<n;i++) p[i] = {a[i], i};
  UnionFind uf(n);
  for(int i=0;i<m;i++) {
    int x, y;
    cin >> x >> y;
    uf.merge(x, y);
  }
  if(m == n-1) {
    cout << 0 << endl;
    return 0;
  }
  if(n < 2 * (n-m-1)) {
    cout << "Impossible" << endl;
    return 0;
  }
  map<int, pll> mp;
  for(int i=0;i<n;i++) {
    int root = uf.root(i);
    if(mp.find(root) == mp.end()) mp[root] = p[i];
    else mp[root] = min(mp[root], p[i]);
  }
  int cnt = 0;
  ll ans = 0;
  for(auto e : mp) {
    ans += e.second.first;
    cnt++;
    a[e.second.second] = INF;
  }
  sort(a.begin(), a.end());
  int q = 2 * (n-m-1) - cnt;
  for(int i=0;i<q;i++) ans += a[i];
  cout << ans << endl;
  return 0;
}
