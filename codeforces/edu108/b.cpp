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
using heap = priority_queue<int, vector<int>, greater<int>>;

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
  void init(int idx, ll val) {
    idx += sz-1;
    t[idx] = val;
    return;
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
  void update(int l, int r, ll val, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(r < a || l > b) return;
    if(l <= a && b <= r) {
      t[now] -= val;
      return;
    }
    update(l, r, val, now*2+1, a, (a+b)/2);
    update(l, r, val, now*2+2, (a+b)/2, b);
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
  ll getElement(int idx) {
    if(idx == 0) return t[0];
    ll res = t[idx];
    return res += getElement((idx-1)/2);
  }
  ll operator[](int idx) {
    idx += sz-1;
    return getElement(idx);
  }
};

void dfs(vector<vector<int>> &g, int x, int y) {
  int h = g.size();
  int w = g[0].size();
  if(x == h-1 && y == w-1) return;
  if(x+1 < h && g[x+1][y] < 0) {
    g[x+1][y] = g[x][y] + y+1;
    dfs(g, x+1, y);
  }
  if(y+1 < w && g[x][y+1] < 0) {
    g[x][y+1] = g[x][y] + x+1;
    dfs(g, x, y+1);
  }
  return;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    int n, m, k;
    cin >> n >> m >> k;
    vector<vector<int>> g(n, vector<int>(m, -1));
    g[0][0] = 0;
    dfs(g, 0, 0);
    // cout << "g[n-1][m-1]: " << g[n-1][m-1] << endl;
    if(g[n-1][m-1] == k) cout << "YES" << endl;
    else cout << "NO" << endl;
  }
  return 0;
}
