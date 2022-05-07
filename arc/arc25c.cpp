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
  int n, m, r, t;
  cin >> n >> m >> r >> t;
  weightedGraph g(n);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;b--;
    g[a].push_back(Edge(b, c));
    g[b].push_back(Edge(a, c));
  }
  ll ans = 0;
  for(int i=0;i<n;i++) {
    vector<ll> d(n, -1);
    heap<pll> nt;
    nt.push({0, i});
    while(!nt.empty()) {
      auto [dist, now] = nt.top();
      nt.pop();
      if(d[now] >= 0) continue;
      d[now] = dist;
      for(auto [to, w] : g[now]) {
        if(d[to] >= 0) continue;
        nt.push({dist+w, to});
      }
    }
    sort(d.begin(), d.end());
    for(int j=1;j<n;j++) {
      ll dt = d[j];
      int wa = 0, ac = n;
      while(ac-wa > 1) {
        int wj = (ac+wa) / 2;
        ll dr = d[wj];
        if(dr * t > dt * r) ac = wj;
        else wa = wj;
      }
      if(ac <= j) ac++;
      ans += n - ac;
    }
  }
  cout << ans << endl;
  return 0;
}
