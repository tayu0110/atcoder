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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
void dfs(int now, vector<int>& d, weightedGraph& t, int par) {
  for(auto e : t[now]) {
    if(d[e.to] >= 0) continue;
    if(e.weight == d[now]) d[e.to] = (e.weight+1) % d.size() + 1;
    else d[e.to] = e.weight;
    dfs(e.to, d, t, now);
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int u, v, c;
    cin >> u >> v >> c;
    u--;v--;
    t[u].push_back(Edge(v, c));
    t[v].push_back(Edge(u, c));
  }
  vector<int> d(n, -1);
  set<int> ck;
  for(int i=0;i<t[0].size();i++) ck.insert(t[0][i].weight);
  for(int i=1;i<n+1;i++) if(ck.find(i) == ck.end()) d[0] = i;
  dfs(0, d, t, 0);
  for(int i=0;i<n;i++) if(d[i] < 0) {
    cout << "No" << endl;
    return 0;
  }
  for(auto e : d) cout << e << endl;
  return 0;
}
