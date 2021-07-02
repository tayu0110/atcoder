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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<string> a(h);
  pii s, g;
  vector<vector<pii>> t(26);
  for(int i=0;i<h;i++) {
    cin >> a[i];
    for(int j=0;j<w;j++) {
      char c = a[i][j];
      if(c == 'S') s = {i, j};
      if(c == 'G') g = {i, j};
      if(c >= 'a' && c <= 'z') t[c-'a'].push_back({i, j});
    }
  }
  vector<vector<bool>> ck(h, vector<bool>(w, false));
  vector<vector<int>> d(h, vector<int>(w, inf));
  d[s.first][s.second] = 0;
  vector<bool> tel(26, false);
  queue<pii> nt;
  nt.push(s);
  while(!nt.empty()) {
    pii now = nt.front();
    nt.pop();
    int y = now.first;
    int x = now.second;
    if(ck[y][x]) continue;
    ck[y][x] = true;
    if(x+1 < w && !ck[y][x+1] && a[y][x+1] != '#') {
      d[y][x+1] = min(d[y][x+1], d[y][x] + 1);
      nt.push({y, x+1});
    }
    if(x-1 >= 0 && !ck[y][x-1] && a[y][x-1] != '#') {
      d[y][x-1] = min(d[y][x-1], d[y][x] + 1);
      nt.push({y, x-1});
    }
    if(y+1 < h && !ck[y+1][x] && a[y+1][x] != '#') {
      d[y+1][x] = min(d[y+1][x], d[y][x] + 1);
      nt.push({y+1, x});
    }
    if(y-1 >= 0 && !ck[y-1][x] && a[y-1][x] != '#') {
      d[y-1][x] = min(d[y-1][x], d[y][x] + 1);
      nt.push({y-1, x});
    }
    char c = a[y][x];
    if(c < 'a' || c > 'z') continue;
    if(tel[c-'a']) continue;
    tel[c-'a'] = true;
    for(int i=0;i<t[c-'a'].size();i++) {
      pii nxt = t[c-'a'][i];
      int ny = nxt.first;
      int nx = nxt.second;
      if(ny == y && nx == x) continue;
      d[ny][nx] = min(d[ny][nx], d[y][x] + 1);
      nt.push({ny, nx});
    }
  }
  // for(int i=0;i<h;i++) {
  //   for(int j=0;j<w;j++) {
  //     cout << d[i][j] << " ";
  //   }
  //   cout << endl;
  // }
  if(d[g.first][g.second] == inf) cout << -1 << endl;
  else cout << d[g.first][g.second] << endl;
  return 0;
}
