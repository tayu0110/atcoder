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
using heap = priority_queue<pii, vector<pii>, greater<pii>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int u, v, l;
    cin >> u >> v >> l;
    u--;v--;
    t[u].push_back(Edge(v, l));
    t[v].push_back(Edge(u, l));
  }
  int ans = inf;
  for(auto g : t[0]) {
    int goal = g.to;
    heap nt;
    nt.push({0, 0});
    vector<int> d(n, inf);
    while(!nt.empty()) {
      auto p = nt.top();
      int now = p.second;
      int nd = p.first;
      nt.pop();
      if(d[now] < inf) continue;
      d[now] = nd;
      if(now == goal) break;
      for(auto e : t[now]) {
        int to = e.to;
        if(now == 0 && to == goal) continue;
        if(d[to] < inf) continue;
        nt.push({nd+e.weight, to});
      }
    }
    if(d[goal] == inf) continue;
    ans = min(ans, d[goal] + (int)g.weight);
  }
  if(ans == inf) ans = -1;
  cout << ans << endl;
  return 0;
}
