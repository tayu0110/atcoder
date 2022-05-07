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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
struct FordFullkerson {
  struct Edge {
    int to;
    int cap;
    int rev;
    Edge() : to{0}, cap{0}, rev{-1} {}
    Edge(int to, int cap, int rev) : to{to}, cap{cap}, rev{rev} {}
    Edge(const Edge &e) : to{e.to}, cap{e.cap}, rev{e.rev} {}
  };
  int sz;
  vector<vector<Edge>> graph;
  vector<bool> used;
  FordFullkerson(int sz) : sz{sz}, graph{vector<vector<Edge>>(sz, vector<Edge>())}, used{vector<bool>(sz, false)} {}
  void set_edge(int from, int to, int cap) { 
    int fsz = graph[from].size();
    int tsz = graph[to].size();
    graph[from].push_back(Edge(to, cap, tsz));
    graph[to].push_back(Edge(from, 0, fsz));
  }
  int flow(int start, int goal) {
    int res = 0;
    while(true) {
      used.assign(sz, false);
      int f = dfs(start, inf, goal);
      if(!f) return res;
      else res += f;
    }
    return 0;
  }
 private:
  int dfs(int now, int f, int goal) {
    if(now == goal) return f;
    used[now] = true;
    for(auto &[to, cap, rev] : graph[now]) {
      if(used[to]) continue;
      if(!cap) continue;
      int res = dfs(to, min(f, cap), goal);
      if(!res) continue;
      graph[to][rev].cap += res;
      cap -= res;
      return res;
    }
    return 0;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, g, e;
  cin >> n >> g >> e;
  vector<int> p(g);
  for(int i=0;i<g;i++) cin >> p[i];
  FordFullkerson ff(n+1);
  for(int i=0;i<e;i++) {
    int a, b;
    cin >> a >> b;
    ff.set_edge(a, b, 1);
    ff.set_edge(b, a, 1);
  }
  for(int i=0;i<g;i++) {
    ff.set_edge(p[i], n, 1);
    ff.set_edge(n, p[i], 1);
  }
  cout << ff.flow(0, n) << endl;
  return 0;
}
