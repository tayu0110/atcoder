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
vector<vector<bool>> ck;
vector<vector<bool>> dp;
bool dfs(int r, int c, vector<string> &s) {
  int h, w;
  h = ck.size();
  w = ck[0].size();
  if(r >= h || c >= w) return false;
  if(ck[r][c]) return dp[r][c];
  ck[r][c] = true;
  if(s[r][c] == '#') return dp[r][c] = false;
  if(r+1 >= h && c+1 >= w) return dp[r][c] = false;
  if(r+1 >= h && c+1 < w) {
    if(s[r][c+1] == '#') return dp[r][c] = false;
    bool f = !dfs(r, c+1, s);
    return dp[r][c] = f;
  } else if(r+1 < h && c+1 >= w) {
    if(s[r+1][c] == '#') return dp[r][c] = false;
    bool f = !dfs(r+1, c, s);
    return dp[r][c] = f;
  } else {
    if(s[r+1][c] == '#' && s[r][c+1] == '#' && s[r+1][c+1] == '#') return dp[r][c] = false;
  }
  bool f = false;
  if(s[r+1][c] != '#') f = f || !dfs(r+1, c, s);
  if(s[r][c+1] != '#') f = f || !dfs(r, c+1, s);
  if(s[r+1][c+1] != '#') f = f || !dfs(r+1, c+1, s);
  return dp[r][c] = f;
}

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
  ck.assign(h, vector<bool>(w, false));
  dp.assign(h, vector<bool>(w, false));
  bool f = dfs(0, 0, s);
  if(f) cout << "First" << endl;
  else cout << "Second" << endl;
  return 0;
}
