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

template<class T>
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
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
  int n;
  cin >> n;
  vector<pair<ll, pii>> p(n);
  for(int i=0;i<n-1;i++) {
    int u, v;
    ll w;
    cin >> u >> v >> w;
    u--;v--;
    p[i] = {w, {u, v}};
  }
  sort(p.begin(), p.end());
  UnionFind uf(n);
  ll ans = 0;
  for(int i=0;i<n;i++) {
    int u = p[i].second.first, v = p[i].second.second;
    ll w = p[i].first;
    ll us = uf.size(u), vs = uf.size(v);
    ans += w * us * vs;
    uf.merge(u, v);
  }
  cout << ans << endl;
  return 0;
}
