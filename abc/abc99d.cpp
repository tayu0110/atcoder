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
void calc(vector<vector<int>> &c, vector<vector<int>> &d, vector<int> &a, int k, int C, vector<int> &color) {
  int n = c.size();
  for(int l=0;l<C;l++) {
    int tot = 0;
    for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
      if((i+j) % 3 != k) continue;
      int col = c[i][j]-1;
      tot += d[col][l];
    }
    if(a[0] >= tot) {
      a[2] = a[1];color[2] = color[1];
      a[1] = a[0];color[1] = color[0];
      a[0] = tot;color[0] = l;
    } else if(a[1] >= tot) {
      a[2] = a[1];color[2] = color[1];
      a[1] = tot;color[1] = l;
    } else if(a[2] > tot) {
      a[2] = tot;color[2] = l;
    }
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, C;
  cin >> n >> C;
  vector<vector<int>> d(C, vector<int>(C, 0));
  for(int i=0;i<C;i++) for(int j=0;j<C;j++) {
    cin >> d[i][j];
  }
  vector<vector<int>> c(n, vector<int>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    cin >> c[i][j];
  }
  vector<int> a1(3, inf), a2(3, inf), a3(3, inf);
  vector<int> c1(3), c2(3), c3(3);
  calc(c, d, a1, 0, C, c1);
  calc(c, d, a2, 1, C, c2);
  calc(c, d, a3, 2, C, c3);
  int ans = inf;
  for(int i=0;i<3;i++) {
    for(int j=0;j<3;j++) {
      if(c1[i] == c2[j]) continue;
      for(int k=0;k<3;k++) {
        if(c3[k] == c1[i] || c3[k] == c2[j]) continue;
        ans = min(ans, a1[i] + a2[j] + a3[k]);
      }
    }
  }
  cout << ans << endl;
  return 0;
}
