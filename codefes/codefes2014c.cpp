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

struct SegmentTree {
  int sz;
  vector<int> t;
  SegmentTree(int n) {
    sz = 1;
    while(sz < n) sz *= 2;
    t.assign(2 * sz - 1, 0);
  }
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] > val) break;
      t[idx] = val;
    }
    return;
  }
  int getMax(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res = max(res, getMax(l, r, 2*now+1, a, (a+b)/2));
    res = max(res, getMax(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  int s, t;
  cin >> s >> t;
  s--;t--;
  weightedGraph g(n);
  for(int i=0;i<m;i++) {
    int x, y, d;
    cin >> x >> y >> d;
    x--;y--;
    g[x].emplace_back(Edge(y, d));
    g[y].emplace_back(Edge(x, d));
  }
  vector<ll> ds(n, -1);
  heap nt;
  nt.push({0, s});
  while(!nt.empty()) {
    int now = nt.top().second;
    ll t = nt.top().first;
    nt.pop();
    if(ds[now] >= 0) continue;
    ds[now] = t;    
    for(int i=0;i<g[now].size();i++) {
      int j = g[now][i].to;
      if(j == t) continue;
      if(ds[j] >= 0) continue;
      ll d = ds[now] + g[now][i].weight;
      nt.push({d, j});
    }
  }
  vector<ll> dt(n, -1);
  nt.push({0, t});
  while(!nt.empty()) {
    int now = nt.top().second;
    ll t = nt.top().first;
    nt.pop();
    if(dt[now] >= 0) continue;
    dt[now] = t;
    for(int i=0;i<g[now].size();i++) {
      int j = g[now][i].to;
      if(j == s) continue;
      if(dt[j] >= 0) continue;
      ll d = dt[now] + g[now][i].weight;
      nt.push({d, j});
    }
  }
  int u = -1;
  for(int i=0;i<n;i++) {
    if(i == s || i == t) continue;
    if(ds[i] == -1 || dt[i] == -1) continue;
    if(ds[i] == dt[i]) u = i + 1;
  }
  cout << u << endl;
  return 0;
}
