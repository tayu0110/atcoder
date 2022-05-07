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
  vector<ll> node, lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.assign(2*n-1, INF); lazy.assign(2*n-1, INF);
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  void evalMin(int now, int l, int r) {
    if(lazy[now] < INF) {
      node[now] = min(node[now], lazy[now]);
      if(r - l > 1) {
        lazy[2*now+1] = min(lazy[now], lazy[2*now+1]);
        lazy[2*now+2] = min(lazy[now], lazy[2*now+2]);
      }
      lazy[now] = INF;
    }
  }
  void updateMin(int a, int b, ll x, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    evalMin(now, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[now] = min(lazy[now], x);
      evalMin(now, l, r);
      return;
    }
    updateMin(a, b, x, 2*now+1, l, (l+r)/2);
    updateMin(a, b, x, 2*now+2, (l+r)/2, r);
    node[now] = min(node[2*now+1], node[2*now+2]);
  }
  ll getMin(int a, int b, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return INF;
    evalMin(now, l, r);
    if(a <= l && r <= b) return node[now];
    ll res = INF;
    res = min(res, getMin(a, b, 2*now+1, l, (l+r)/2));
    res = min(res, getMin(a, b, 2*now+2, (l+r)/2, r));
    return res;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, l;
  cin >> n >> l;
  vector<tuple<int, int, ll>> p(n);
  for(int i=0;i<n;i++) {
    int le, ri, c;
    cin >> le >> ri >> c;
    p[i] = {le, ri, c};
  }
  sort(p.begin(), p.end());
  LazySegmentTree st(l+1);
  for(int i=0;i<n;i++) {
    auto [L, R, c] = p[i];
    ll k = c;
    if(L) k += st.getMin(L, R+1);
    st.updateMin(L, R+1, k);
  }
  cout << st.getMin(l, l+1) << endl;
  return 0;
}
