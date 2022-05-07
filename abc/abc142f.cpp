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
vector<int> d;
bool dfs(int now, int dist, int mx, int start, Graph &t) {
  if(now == start && dist == mx) return true;
  for(auto to : t[now]) {
    if(d[to] != dist+1 && to != start) continue;
    if(dfs(to, dist+1, mx, start, t)) {
      cout << now+1 << endl;
      return true;
    }
  }
  return false;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  vector<int> in(n), out(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    in[b]++;
    out[a]++;
  }
  int mx = inf;
  int idx = -1;
  for(int i=0;i<n;i++) {
    if(!in[i] || !out[i]) continue;
    d.assign(n, -1);
    queue<pii> nt;
    nt.push({i, 0});
    while(!nt.empty()) {
      auto [now, dist] = nt.front();
      nt.pop();
      if(dist > mx) break;
      if(now == i && dist) {
        mx = dist;
        idx = i;
        break;
      }
      if(d[now] >= 0) continue;
      d[now] = dist;
      for(auto to : t[now]) {
        nt.push({to, dist+1});
      }
    }
  }
  if(idx < 0) {
    cout << -1 << endl;
    return 0;
  }
  d.assign(n, -1);
  queue<pii> nt;
  nt.push({idx, 0});
  while(!nt.empty()) {
    auto [now, dist] = nt.front();
    nt.pop();
    if(d[now] >= 0) continue;
    d[now] = dist;
    for(auto to : t[now]) nt.push({to, dist+1});
  }
  cout << mx << endl;
  dfs(idx, 0, mx, idx, t);
  return 0;
}
