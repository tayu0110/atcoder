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
const ld PI = acos(-1);
struct LazySegmentTree {
 private:
  int n;
  vector<map<int, int>> node, lazy;
  vector<int> fac;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.resize(2*n-1); lazy.resize(2*n-1);
  }
  LazySegmentTree(int sz) {
    init(sz);
    int mx = 1000100;
    fac.assign(mx, -1);
    for(int i=2;i<mx;i++) {
      if(fac[i] >= 0) continue;
      for(int j=1;i*j<mx;j++) {
        if(fac[i*j] < 0) fac[i*j] = i;
      }
    }
  }
  void eval(int now, int l, int r) {
    if(!lazy[now].empty()) {
      for(auto [f, s] : lazy[now]) {
        node[now][f] += s;
        if(node[now][f] % 3 == 0) node[now].erase(f);
      }
      if(r - l > 1) {
        // lazy[2*now+1] += lazy[now] / 2;
        // lazy[2*now+2] += lazy[now] / 2;
      }
      lazy[now].clear();
    }
  }
  void add(int a, int b, ll x, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    eval(k, l, r);
    if(b <= l || r <= a) return;
    // if(a <= l && r <= b) {
    //   lazy[k] += (r-l) * x;
    //   eval(k, l, r);
    // } else {
    //   add(a, b, x, 2*k+1, l, (l+r)/2);
    //   add(a, b, x, 2*k+2, (l+r)/2, r);
    //   node[k] = node[2*k+1] + node[2*k+2];
    // }
  }
  ll getSum(int a, int b, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return 0;
    eval(k, l, r);
    // if(a <= l && r <= b) return node[k];
    ll res = 0;
    res += getSum(a, b, 2*k+1, l, (l+r)/2);
    res += getSum(a, b, 2*k+2, (l+r)/2, r);
    return res;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];

  while(q--) {
    int l, r;
    cin >> l >> r;
    l--;r--;

  }
  return 0;
}
