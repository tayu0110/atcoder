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
  long long weight, cost;
  Edge() : to(0), weight(0), cost(0) {}
  Edge(int to, long long weight, long long cost) : to(to), weight(weight), cost(cost) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight), cost(e.cost) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};
using weightedGraph = vector<vector<Edge>>;
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
  void swap(heap<T> nt) { pq.swap(nt.pq); }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, s;
  cin >> n >> m >> s;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int u, v, a, b;
    cin >> u >> v >> a >> b;
    u--;v--;
    t[u].push_back(Edge(v, b, a));
    t[v].push_back(Edge(u, b, a));
  }
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    int c, d;
    cin >> c >> d;
    p[i] = {c, d};
  }
  const int max_cost = n * 50 + 10;
  s = min(max_cost-1, s);
  vector<vector<ll>> dist(n, vector<ll>(n * 50 + 10, INF));
  heap<tuple<ll, ll, int>> nt;
  nt.push({0, s, 0});
  while(!nt.empty()) {
    auto [d, m, now] = nt.top();
    nt.pop();
    // DEBUG(d);DEBUG(m);DEBUG_EN(now);
    if(m >= max_cost) continue;
    if(dist[now][m] <= d) continue;
    dist[now][m] = d;
    nt.push({d + p[now].second, m + p[now].first, now});
    for(auto [to, w, cost] : t[now]) {
      if(m < cost) continue;
      if(dist[to][m-cost] <= d + w) continue;
      nt.push({d + w, m - cost, to});
    }
  }
  for(int i=1;i<n;i++) {
    ll mn = INF;
    for(auto e : dist[i]) mn = min(mn, e);
    // DEBUG(i);
    // print_with_space(dist[i]);
    cout << mn << endl;
  }
  return 0;
}
