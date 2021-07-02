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
using heap = priority_queue<pll, vector<pll>, greater<pll>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

void f(int start, weightedGraph& g, vector<ll>& d) {
  heap nt;
  nt.push({0, start});
  while(!nt.empty()) {
    pll p = nt.top();
    nt.pop();
    int now = p.second;
    int nd = p.first;
    if(d[now] >= 0) continue;
    d[now] = nd;
    for(auto e : g[now]) {
      if(d[e.to] >= 0) continue;
      nt.push({nd+e.weight, e.to});
    }
  }
  return;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<ll> s(n, -1), g(n, -1);
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;b--;
    t[a].push_back(Edge(b, c));
    t[b].push_back(Edge(a, c));
  }
  f(0, t, s);
  f(n-1, t, g);
  for(int i=0;i<n;i++) {
    cout << s[i] + g[i] << endl;
  }
  return 0;
}
