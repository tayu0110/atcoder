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
using heap = priority_queue<pll, vector<pll>, greater<pll>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
void dijkstra(int start, weightedGraph &t, vector<int> &d) {
  heap nt;
  nt.push({0, start});
  while(!nt.empty()) {
    auto p = nt.top();
    int now = p.second;
    ll nd = p.first;
    nt.pop();
    if(d[now] >= 0) continue;
    d[now] = nd;
    for(auto e : t[now]) {
      int to = e.to;
      if(d[to] >= 0) continue;
      nt.push({nd + e.weight, to});
    }
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  ll t;
  cin >> n >> m >> t;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  weightedGraph g(n);
  weightedGraph h(n);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;b--;
    g[a].push_back(Edge(b, c));
    h[b].push_back(Edge(a, c));
  }
  vector<int> fd(n, -1), bd(n, -1);
  dijkstra(0, g, fd);
  dijkstra(0, h, bd);
  vector<ll> ans(n);
  for(int i=0;i<n;i++) {
    if(fd[i] == -1 || bd[i] == -1) continue;
    ll rest = t - fd[i] - bd[i];
    ans[i] = rest * a[i];
  }
  cout << *max_element(ans.begin(), ans.end()) << endl;
  return 0;
}
