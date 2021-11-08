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
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<pll, vector<pll>, greater<pll>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
void dijkstra(int start, vector<ll> &d, weightedGraph &t) {
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
      ll weight = e.weight;
      if(d[to] >= 0) continue;
      nt.push({nd+weight, to});
    }
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, s, t;
  cin >> n >> m >> s >> t;
  s--;t--;
  weightedGraph ga(n), gb(n);
  for(int i=0;i<m;i++) {
    int u, v;
    ll a, b;
    cin >> u >> v >> a >> b;
    u--;v--;
    ga[u].push_back(Edge(v, a));
    ga[v].push_back(Edge(u, a));
    gb[u].push_back(Edge(v, b));
    gb[v].push_back(Edge(u, b));
  }
  vector<ll> da(n, -1), db(n, -1);
  dijkstra(s, da, ga);
  dijkstra(t, db, gb);
  vector<ll> d(n);
  multiset<ll> ck;
  for(int i=0;i<n;i++) d[i] = da[i] + db[i], ck.insert(d[i]);
  const ll w = 1e15;
  for(int i=0;i<n;i++) {
    ll c = *ck.begin();
    cout << w - c << endl;
    auto it = ck.lower_bound(d[i]);
    ck.erase(it);
  }
  return 0;
}
