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
int n, m;
set<pii> ck;
vector<int> d;
bool dfs(int now, int par, Graph &t) {
  if(now == n-1) return true;
  for(auto e : t[now]) {
    if(e == par) continue;
    if(d[e] - d[now] != 1) continue;
    if(dfs(e, now, t)) {
      ck.insert({now, e});
      return true;
    }
  }
  return false;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n >> m;
  vector<pii> p(m);
  Graph g(n);
  for(int i=0;i<m;i++) {
    int s, t;
    cin >> s >> t;
    s--;t--;
    p[i] = {s, t};
    g[s].push_back(t);
  }
  d.assign(n, -1);
  queue<pii> nt;
  nt.push({0, 0});
  while(!nt.empty()) {
    auto [now, nd] = nt.front();
    nt.pop();
    if(d[now] >= 0) continue;
    d[now] = nd;
    if(now == n-1) break;
    for(auto e : g[now]) {
      if(d[e] >= 0) continue;
      nt.push({e, nd+1});
    }
  }
  if(d[n-1] < 0) {
    for(int i=0;i<m;i++) cout << -1 << endl;
    return 0;
  }
  dfs(0, -1, g);
  for(int i=0;i<m;i++) {
    if(ck.find(p[i]) == ck.end()) {
      cout << d[n-1] << endl;
      continue;
    }
    int from = p[i].first, to = p[i].second;
    vector<int> dist(n, -1);
    nt.push({0, 0});
    while(!nt.empty()) {
      auto [now, nd] = nt.front();
      nt.pop();
      if(dist[now] >= 0) continue;
      dist[now] = nd;
      if(now == n-1) break;
      for(auto e : g[now]) {
        if(dist[e] >= 0) continue;
        if(now == from && e == to) continue;
        nt.push({e, nd+1});
      }
    }
    cout << dist[n-1] << endl;
  }
  return 0;
}
