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
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  for(int i=0;i<h;i++) {
    cin >> s[i];
  }
  vector<vector<ll>> res(h, vector<ll>(w));
  vector<vector<ll>> t(h, vector<ll>(w));
  vector<vector<ll>> y(h, vector<ll>(w));
  vector<vector<ll>> n(h, vector<ll>(w));
  res[0][0] = 1;
  t[0][0] = 1;
  y[0][0] = 1;
  n[0][0] = 1;
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(s[i][j] == '#') continue;
      if(i==0 && j==0) continue;
      ll k = 0, l = 0, m = 0;
      if(i-1 >= 0 && j-1 >= 0) m = n[i-1][j-1];
      if(i-1 >= 0) k = t[i-1][j];
      if(j-1 >= 0) l = y[i][j-1];
      res[i][j] = (k + l + m) % MOD;
      t[i][j] = (res[i][j] + k) % MOD;
      y[i][j] = (res[i][j] + l) % MOD;
      n[i][j] = (res[i][j] + m) % MOD;
    }
  }
  cout << res[h-1][w-1] << endl;
  return 0;
}
