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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

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
  // update the maximum value of the interval
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
  // update the sum of the interval
  void update(int l, int r, ll val, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(r < a || l > b) return;
    if(a <= l && r <= b) {
      t[now] += val;
      update(l, r, val, now*2+1, a, (a+b)/2);
      update(l, r, val, now*2+2, (a+b)/2, b);
    }
    return;
  }
  // get the sum of interval
  int getSum(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res += getSum(l, r, 2*now+1, a, (a+b)/2);
    res += getSum(l, r, 2*now+2, (a+b)/2, b);
    return res;
  }
  // get the maximum value of the interval
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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> p(n);
  map<int, int> mp;
  for(int i=0;i<n;i++) {
    int w, h;
    cin >> w >> h;
    p[i] = {w, h};
    mp[h] = 0;
  }
  int cnt = 1;
  for(auto &[f, s] : mp) s = cnt++;
  for(auto &[f, s] : p) s = mp[s];
  sort(p.begin(), p.end(), [](pii lhs, pii rhs) {
    if(lhs.first == rhs.first) return lhs.second > rhs.second;
    return lhs.first < rhs.first;
  });
  SegmentTree st(cnt);
  for(int i=0;i<n;i++) {
    auto [f, s] = p[i];
    int mx = st.getMax(0, s);
    int now = st.getMax(s, s+1);
    st.update(s, max(mx+1, now));
  }
  cout << st.getMax(0, cnt) << endl;
  return 0;
}
