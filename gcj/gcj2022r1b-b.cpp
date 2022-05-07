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
const ld PI = acos(-1);

ll dfs(int idx, ll now, vector<vector<ll>> &x, map<pll, ll> &mp) {
  if(idx == x.size()) return 0;
  if(mp.find({idx, now}) != mp.end()) return mp[{idx, now}];
  if(now <= x[idx][0]) return mp[{idx, now}] = x[idx].back() - now + dfs(idx+1, x[idx].back(), x, mp);
  if(now >= x[idx].back()) return mp[{idx, now}] = now - x[idx][0] + dfs(idx+1, x[idx][0], x, mp);
  return mp[{idx, now}] = x[idx].back() - x[idx][0] + min(abs(now-x[idx][0])+dfs(idx+1, x[idx].back(), x, mp), abs(x[idx].back()-now)+dfs(idx+1, x[idx][0], x, mp));
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    int n, p;
    cin >> n >> p;
    vector<vector<ll>> x(n, vector<ll>(p, 0));
    for(int j=0;j<n;j++) {
      for(int k=0;k<p;k++) cin >> x[j][k];
      sort(x[j].begin(), x[j].end());
    }
    map<pll, ll> mp;
    printf("Case #%d: %lld\n", i, dfs(0, 0, x, mp));
  }
  return 0;
}
