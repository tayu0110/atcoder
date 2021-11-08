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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  weightedGraph t(n);
  for(int i=0;i<n-1;i++) {
    int u, v;
    ll w;
    cin >> u >> v >> w;
    u--;v--;
    t[u].push_back(Edge(v, w));
    t[v].push_back(Edge(u, w));
  }
  vector<ll> d(n, -1);
  queue<pll> nt;
  nt.push({0, 0});
  while(!nt.empty()) {
    auto [nd, now] = nt.front();
    nt.pop();
    if(d[now] >= 0) continue;
    d[now] = nd;
    for(auto [to, w] : t[now]) {
      if(d[to] >= 0) continue;
      nt.push({nd+w, to});
    }
  }
  for(auto e : d) cout << e % 2 << endl;
  return 0;
}
