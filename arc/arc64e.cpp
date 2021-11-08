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
struct Edge {
  int to;
  long double weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long double weight) : to(to), weight(weight) {}
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
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll sx, sy, tx, ty;
  int n;
  cin >> sx >> sy >> tx >> ty >> n;
  n += 2;
  vector<tuple<ld, ld, ld>> p(n);
  p[0] = {sx, sy, 0};
  p[n-1] = {tx, ty, 0};
  for(int i=1;i<n-1;i++) {
    ld x, y, r;
    cin >> x >> y >> r;
    p[i] = {x, y, r};
  }
  weightedGraph t(n);
  for(int i=0;i<n;i++) {
    auto [x, y, r] = p[i];
    for(int j=i+1;j<n;j++) {
      auto [px, py, pr] = p[j];
      ld tmp = (px-x)*(px-x) + (py-y)*(py-y);
      ld dist = sqrtl(tmp) - r - pr;
      if(dist < 0) dist = 0;
      t[i].push_back(Edge(j, dist));
      t[j].push_back(Edge(i, dist));
    }
  }
  heap<pair<ld, int>> hp;
  hp.push({0, 0});
  vector<ld> d(n, -10);
  while(!hp.empty()) {
    auto [nd, now] = hp.top();
    hp.pop();
    if(d[now] >= 0) continue;
    d[now] = nd;
    for(auto [to, w] : t[now]) {
      if(d[to] >= 0) continue;
      hp.push({nd + w, to});
    }
  }
  cout << d[n-1] << endl;
  return 0;
}
