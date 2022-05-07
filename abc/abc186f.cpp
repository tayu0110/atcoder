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
template<class T>
class FenwickTree {
  vector<T> t;
  int sz;
public: 
  FenwickTree(int n) : sz(n+1), t(vector<T>(n+1, 0)) {}
  // fix the 0-based array to the 1-based array
  FenwickTree(vector<T> &s) : sz(s.size() + 1) {
    t.assign(sz, 0);
    for(int i=0;i<s.size();i++) add(i, s[i]);
  }
  // add(idx, val) => add val to idx th element
  // idx : 0-based index
  void add(int idx, T val) {
    idx++;
    while(idx < sz) {
      t[idx] += val;
      idx += idx & -idx;
    }
  }
  // getSum(l, r) => get the sum of [l, r)
  // l, r : 0-based index
  T getSum(int l, int r) {
    // need not fix l and r to 1-based index
    // because subGetSum() needs a 0-based index argument.
    return subGetSum(r) - subGetSum(l);
  }
  // lower_bound(val) => return 0-based index
  int lower_bound(T val) {
    if(val <= 0) return 0;
    int now = 0;
    int n = 1;
    while(n<<1 <= sz) n <<= 1;
    for(int i=n;i>0;i>>=1) {
      if(now+i < sz && t[now+i] < val) {
        val -= t[now+i];
        now += i;
      }
    }
    return now;
  }
  // upper_bound(val) => return 0-based index
  int upper_bound(T val) {
    if(val <= 0) return 0;
    int now = 0;
    int n = 1;
    while(n<<1 <= sz) n <<= 1;
    for(int i=n;i>0;i>>=1) {
      if(now+i < sz && t[now+i] <= val) {
        val -= t[now+i];
        now += i;
      }
    }
    return now;
  }
private:
  // subGetSum(r) => get the sum of [0, r)
  // r : 0-based index
  T subGetSum(int r) {
    if(r < 0) return 0;
    if(r >= sz) r = sz-1;
    T res = 0;
    while(r > 0) {
      res += t[r];
      r -= r & -r;
    }
    return res;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll H, W, m;
  cin >> H >> W >> m;
  if(!m) {
    cout << H * W << endl;
    return 0;
  }
  vector<vector<int>> v(W+1), h(H+1);
  for(int i=0;i<m;i++) {
    int x, y;
    cin >> x >> y;
    h[x].push_back(y);
    v[y].push_back(x);
  }
  ll ans = 0;
  for(int i=1;i<H+1;i++) {
    if(!h[i].size()) {
      ans += W;
      continue;
    }
    int mn = *min_element(h[i].begin(), h[i].end());
    if(mn == 1) break;
    ans += mn - 1;
  }
  // DEBUG_EN(ans);
  FenwickTree<ll> ft(H+1);
  for(int i=1;i<W+1;i++) {
    if(!v[i].size()) {
      ans += ft.getSum(0, H+1);
      // DEBUG_EN(ft.getSum(0, H+1));
      continue;
    }
    int mn = *min_element(v[i].begin(), v[i].end());
    if(mn == 1) break;
    ans += ft.getSum(0, mn);
    if(i == 1) {
      for(int i=mn;i<H+1;i++) ft.add(i, 1);
    } else {
      for(int j=0;j<v[i].size();j++) {
        int idx = v[i][j];
        if(ft.getSum(idx, idx+1)) continue;
        ft.add(idx, 1);
      }
    }
  }
  cout << ans << endl;
  return 0;
}
