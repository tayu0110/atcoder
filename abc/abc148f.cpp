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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, u, v;
  cin >> n >> u >> v;
  u--;v--;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<int> da(n, -1), dt(n, -1);
  queue<int> nt;
  nt.push(v);
  da[v] = 0;
  while(!nt.empty()) {
    int now = nt.front();
    nt.pop();
    for(auto e : t[now]) {
      if(da[e] >= 0) continue;
      da[e] = da[now] + 1;
      nt.push(e);
    }
  }
  nt.push(u);
  dt[u] = 0;
  while(!nt.empty()) {
    int now = nt.front();
    nt.pop();
    for(auto e : t[now]) {
      if(dt[e] >= 0) continue;
      if(dt[now]+1 >= da[e]) continue;
      dt[e] = dt[now] + 1;
      nt.push(e);
    }
  }
  int ans = 0;
  int mx = 0;
  for(int i=0;i<n;i++) {
    if(dt[i] < 0) continue;
    ans = max(ans, da[i]-1);
  }
  cout << ans << endl;
  return 0;
}
