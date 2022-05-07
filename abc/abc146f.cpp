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

using namespace std;

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
struct LazySegmentTree {
 private:
  int n;
  vector<pll> node, lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.assign(2*n-1, {inf, -1}); lazy.assign(2*n-1, {inf, -1});
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  void evalMax(int now, int l, int r) {
    if(lazy[now].first < inf) {
      node[now] = min(node[now], lazy[now]);
      if(r - l > 1) {
        lazy[2*now+1] = min(lazy[now], lazy[2*now+1]);
        lazy[2*now+2] = min(lazy[now], lazy[2*now+2]);
      }
      lazy[now] = {inf, -1};
    }
  }
  void updateMax(int a, int b, pll x, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    evalMax(now, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[now] = min(lazy[now], x);
      evalMax(now, l, r);
      return;
    }
    updateMax(a, b, x, 2*now+1, l, (l+r)/2);
    updateMax(a, b, x, 2*now+2, (l+r)/2, r);
    node[now] = min(node[2*now+1], node[2*now+2]);
  }
  pll getMax(int a, int b, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return {inf, -1};
    evalMax(now, l, r);
    if(a <= l && r <= b) return node[now];
    pll res = {inf, -1};
    res = min(res, getMax(a, b, 2*now+1, l, (l+r)/2));
    res = min(res, getMax(a, b, 2*now+2, (l+r)/2, r));
    return res;
  }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, m;
  string s;
  cin >> n >> m >> s;
  LazySegmentTree st(n+1);
  st.updateMax(0, 1, {0, 0});
  for(int i=0;i<n;i++) {
    if(s[i] == '1') continue;
    auto now = st.getMax(i, i+1);
    if(now.first == inf) continue;
    st.updateMax(i, min(i+m+1, n+1), make_pair(now.first+1, i));
  }
  auto now = st.getMax(n, n+1);
  if(now.first == inf) {
    cout << -1 << endl;
    return 0;
  }
  vector<int> res;
  int nt = n;
  while(now.second >= 0) {
    res.push_back(nt - now.second);
    nt = now.second;
    if(!now.second) break;
    now = st.getMax(nt, nt+1);
  }
  reverse(res.begin(), res.end());
  for(int i=0;i<res.size();i++) {
    if(i) cout << " "; cout << res[i];
  }
  cout << endl;
  return 0;
}
