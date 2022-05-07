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
      if(used[to] || !cap) continue;
      int res = dfs(to, min(f, cap), goal);
      if(!res) continue;
      graph[to][rev].cap += res;
      cap -= res;
      return res;
    }
    return 0;
  }
};
int n, m;
bool solve(int tar, vector<pii> p) {
  set<pii> ck;
  for(auto e : p) ck.insert(e);
  for(int i=0;i<n;i++) {
    if(i == tar) continue;
    if(ck.find({tar, i}) == ck.end() && ck.find({i, tar}) == ck.end()) ck.insert({tar, i});
  }
  int win = 0;
  for(auto [w, l] : ck) if(w == tar) win++;
  if(!win) return false;
  map<pii, int> pt;
  const pii start = {-1, -1};
  const pii goal = {inf, inf};
  int cnt = 0;
  for(int i=0;i<n;i++) pt[{i, -1}] = cnt++;
  for(int i=0;i<n;i++) for(int j=i+1;j<n;j++) pt[{i, j}] = cnt++;
  pt[start] = cnt++;
  pt[goal] = cnt++;
  FordFullkerson ff(cnt);
  for(int i=0;i<n;i++) for(int j=i+1;j<n;j++) {
    ff.set_edge(pt[start], pt[{i, j}], 1);
    if(ck.find({i, j}) != ck.end()) ff.set_edge(pt[{i, j}], pt[{i, -1}], 1);
    else if(ck.find({j, i}) != ck.end()) ff.set_edge(pt[{i, j}], pt[{j, -1}], 1);
    else {
      ff.set_edge(pt[{i, j}], pt[{i, -1}], 1);
      ff.set_edge(pt[{i, j}], pt[{j, -1}], 1);
    }
  }
  for(int i=0;i<n;i++) {
    if(i == tar) ff.set_edge(pt[{tar, -1}], pt[goal], win);
    else ff.set_edge(pt[{i, -1}], pt[goal], win-1);
  }
  int flow = ff.flow(pt[start], pt[goal]);
  return (flow == n * (n-1) / 2);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n >> m;
  vector<pii> p(m);
  for(int i=0;i<m;i++) {
    int w, l;
    cin >> w >> l;
    w--; l--;
    p[i] = {w, l};
  }
  vector<int> ans;
  for(int i=0;i<n;i++) {
    if(solve(i, p)) ans.push_back(i+1);
  }
  for(int i=0;i<ans.size();i++) {
    if(i) cout << " "; cout << ans[i];
  }
  cout << endl;
  return 0;
}
