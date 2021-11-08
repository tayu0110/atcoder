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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

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
  Graph g(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    g[u].push_back(v);
  }
  int s, t;
  cin >> s >> t;
  s--;t--;
  vector<set<int>> ck(n);
  heap nt;
  nt.push({0, s});
  ck[s].insert(0);
  while(!nt.empty()) {
    auto p = nt.top();
    int now = p.second;
    int d = p.first;
    nt.pop();
    for(auto e : g[now]) {
      int nd = d+1;
      if(e == t) {
        if(nd % 3 == 0) {
          cout << nd / 3 << endl;
          return 0;
        }
      }
      if(ck[e].find(nd % 3) == ck[e].end()) {
        nt.push({nd, e});
        ck[e].insert(nd % 3);
      }
    }
  }
  cout << -1 << endl;
  return 0;
}
