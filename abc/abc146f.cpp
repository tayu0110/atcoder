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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

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
    t.assign(2 * sz - 1, inf);
  }
  // update the maximum value of the interval
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] < val) break;
      t[idx] = val;
    }
    return;
  }
  // get the maximum value of the interval
  int getMin(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return inf;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return inf;
    if(l <= a && r >= b) return t[now];
    int res = inf;
    res = min(res, getMin(l, r, 2*now+1, a, (a+b)/2));
    res = min(res, getMin(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  string s;
  cin >> n >> m >> s;
  vector<int> d(n+1, inf);
  d[n] = 0;
  SegmentTree st(n+1);
  st.update(n, 0);
  for(int i=n-1;i>=0;i--) {
    if(s[i] == '1') continue;
    int mn = st.getMin(i, i+m+1);
    if(mn == inf) continue;
    d[i] = mn + 1;
    st.update(i, d[i]);
  }
  if(d[0] == inf) {
    cout << -1 << endl;
    return 0;
  }
  int prev = 0;
  int cnt = d[0]-1;
  vector<int> ans;
  for(int i=0;i<n+1;i++) {
    if(d[i] == cnt) {
      cnt--;
      ans.push_back(i-prev);
      prev = i;
    }
  }
  for(int i=0;i<ans.size();i++) {
    cout << ans[i];
    if(i != ans.size()-1) cout << " ";
  }
  cout << endl;
  return 0;
}
