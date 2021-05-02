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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> p(n);
  for(int i=0;i<n;i++) {
    cin >> p[i];
  }
  vector<vector<int>> ans(n, vector<int>(n));
  for(int i=0;i<n;i++) ans[i][i] = p[i];
  for(int i=0;i<n;i++) {
    int cnt = 1;
    queue<pii> nt;
    nt.push({i, i});
    while(cnt != p[i]) {
      pii now = nt.front();
      nt.pop();
      int x = now.first;
      int y = now.second;
      if(x-1 >= 0 && ans[y][x-1] == 0) {
        ans[y][x-1] = p[i];
        cnt++;
        nt.push({x-1, y});
        continue;
      }
      if(y+1 < n && ans[y+1][x] == 0) {
        ans[y+1][x] = p[i];
        cnt++;
        nt.push({x, y+1});
        continue;
      }
      if(x+1 < n && ans[y][x+1] == 0) {
        ans[y][x+1] = p[i];
        cnt++;
        nt.push({x+1, y});
        continue;
      }
      if(y-1 >= 0 && ans[y-1][x] == 0) {
        ans[y-1][x] = p[i];
        cnt++;
        nt.push({x, y-1});
      }
    }
  }
  for(int i=0;i<n;i++) {
    for(int j=0;j<i+1;j++) {
      cout << ans[i][j];
      if(j != i) cout << " ";
    }
    cout << endl;
  }
  return 0;
}
