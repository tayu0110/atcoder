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
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
void dfs(int now, weightedGraph &t, vector<bool> &ck) {
  if(ck[now]) return;
  ck[now] = true;
  for(auto [to, weight] : t[now]) {
    if(ck[to]) continue;
    dfs(to, t, ck);
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, p;
  cin >> n >> m >> p;
  weightedGraph t(n), r(n);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;b--;
    int cost = c-p;
    t[a].push_back(Edge(b, cost));
    r[b].push_back(Edge(a, cost));
  }
  vector<bool> s(n, false), g(n, false);
  dfs(0, t, s);
  dfs(n-1, r, g);
  vector<bool> reachable(n, false);
  for(int i=0;i<n;i++) reachable[i] = s[i] && g[i];
  vector<ll> c(n, -INF);
  c[0] = 0;
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(!reachable[j]) continue;
      if(c[j] == -INF) continue;
      for(auto [to, weight] : t[j]) {
        if(!reachable[to]) continue;
        if(c[to] < c[j] + weight) {
          c[to] = c[j] + weight;
          if(i == n-1) {
            cout << -1 << endl;
            return 0;
          }
        }
      }
    }
  }
  cout << max(c[n-1], 0LL) << endl;
  return 0;
}
