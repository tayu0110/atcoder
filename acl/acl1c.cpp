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

template< typename Cap, typename Cost >
struct MinCostFlow {
  const Cost INF;
  struct Edge {
    int to;
    Cap cap;
    Cost cost;
    int rev;
  };
  vector<vector<Edge>> graph;
  vector<Cost> potential, min_cost;
  vector<int> prevv, preve;
  MinCostFlow(int sz) : graph(sz), INF(numeric_limits<Cost>::max()) {}
  void add_edge(int from, int to, Cap cap, Cost cost) {
    graph[from].emplace_back((Edge) {to, cap, cost, (int) graph[to].size()});
    graph[to].emplace_back((Edge) {from, 0, -cost, (int) graph[from].size()-1});
  }
  Cost min_cost_flow(int s, int t, Cap f) {
    int sz = (int)graph.size();
    Cost ret = 0;
    using Pi = pair<Cost, int>;
    priority_queue<Pi, vector<Pi>, greater<Pi>> nt;
    potential.assign(sz, 0);
    preve.assign(sz, -1);
    prevv.assign(sz, -1);
    while(f > 0) {
      min_cost.assign(sz, INF);
      nt.emplace(0, s);
      min_cost[s] = 0;
      while(!nt.empty()) {
        auto [c, now] = nt.top();
        Pi p = nt.top();
        nt.pop();
        if(min_cost[now] < c) continue;
        for(int i=0;i<graph[now].size();i++) {
          Edge &e = graph[now][i];
          Cost nextCost = min_cost[now] + e.cost + potential[now] - potential[e.to];
          if(e.cap > 0 && min_cost[e.to] > nextCost) {
            min_cost[e.to] = nextCost;
            prevv[e.to] = p.second, preve[e.to] = i;
            nt.emplace(min_cost[e.to], e.to);
          }
        }
      }
      if(min_cost[t] == INF) return -1;
      for(int v=0;v<sz;v++) potential[v] += min_cost[v];
      Cap addflow = f;
      for(int v=t;v!=s;v=prevv[v]) addflow = min(addflow, graph[prevv[v]][preve[v]].cap);
      f -= addflow;
      ret += addflow * potential[t];
      for(int v = t; v != s; v = prevv[v]) {
        Edge &e = graph[prevv[v]][preve[v]];
        e.cap -= addflow;
        graph[v][e.rev].cap += addflow;
      }
    }
    return ret;
  }
};

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  MinCostFlow<ll, ll> mcf(n*m+2);
  int start = n*m, goal = n*m+1, cnt = 0;
  vector<int> dx = {1, 0};
  vector<int> dy = {0, 1};
  vector<string> s(n);
  auto f = [m](int r, int c) { return r*m + c; };
  for(int i=0;i<n;i++) {
    cin >> s[i];
    for(int j=0;j<m;j++) {
      if(s[i][j] == '#') continue;
      if(s[i][j] == 'o') mcf.add_edge(start, f(i, j), 1, 0), cnt++;
      mcf.add_edge(f(i, j), goal, 1, 0);
      for(int k=0;k<2;k++) {
        int ni = i + dy[k];
        int nj = j + dx[k];
        if(ni < 0 || ni >= n || nj < 0 || nj >= m) continue;
        if(s[ni][nj] == '#') continue;
        mcf.add_edge(f(i, j), f(ni, nj), mcf.INF, -1);
      }
    }
  }
  int ans = mcf.min_cost_flow(start, goal, cnt);
  cout << -ans << endl;
  return 0;
}
