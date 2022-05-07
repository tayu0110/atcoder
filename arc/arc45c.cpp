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

using namespace std;

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
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  ll x;
  cin >> n >> x;
  weightedGraph t(n);
  for(int i=0;i<n-1;i++) {
    int x, y, c;
    cin >> x >> y >> c;
    x--; y--;
    t[x].push_back(Edge(y, c));
    t[y].push_back(Edge(x, c));
  }
  queue<pll> nt;
  nt.push({0, 0});
  vector<ll> d(n, -1);
  while(!nt.empty()) {
    auto [now, c] = nt.front(); nt.pop();
    if(d[now] >= 0) continue;
    d[now] = c;
    for(auto [to, w] : t[now]) {
      if(d[to] >= 0) continue;
      ll nc = c ^ w;
      nt.push({to, nc});
    }
  }
  map<ll, ll> mp;
  for(int i=0;i<n;i++) mp[d[i]]++;
  ll ans = 0;
  for(auto [f, s] : mp) {
    ll k = x ^ f;
    if(f > k) continue;
    if(mp.find(k) == mp.end()) continue;
    if(f == k) {
      ans += s * (s-1) / 2;
    } else {
      ans += s * mp[k];
    }
  }
  cout << ans << endl;
  return 0;
}
