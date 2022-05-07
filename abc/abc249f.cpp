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
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<pll> p(n+1);
  p[0] = {1, 0};
  FenwickTree<ll> pnum(n), psum(n), mnum(n), msum(n);
  map<ll, int> mp;
  for(int i=1;i<n+1;i++) {
    int t, y;
    cin >> t >> y;
    p[i] = {t, y};
    if(t == 2) mp[y]++;
  }
  int pcnt = 0, mcnt = n-1;
  vector<ll> prev(n), mrev(n);
  for(auto &[f, s] : mp) {
    if(f < 0) {
      s = mcnt;
      mrev[mcnt--] = -f;
    } else {
      s = pcnt;
      prev[pcnt++] = f;
    }
  }
  ll ans = -INF;
  if(k == n) ans = 0;
  for(int i=n;i>=0;i--) {
    auto [t, y] = p[i];
    if(t == 1) {
      ll need = n-k-i;
      if(need <= 0) ans = max(ans, y);
      int pn = pnum.getSum(0, n);
      if(need > pn + mnum.getSum(0, n)) continue;
      if(pn >= need) {
        ans = max(ans, y + psum.getSum(0, n));
      } else {
        ll res = y + psum.getSum(0, n);
        need -= pn;
        int pos = mnum.lower_bound(need);
        res -= msum.getSum(0, pos);
        need -= mnum.getSum(0, pos);
        res -= need * mrev[pos];
        ans = max(ans, res);
      }
    } else {
      if(y < 0) {
        mnum.add(mp[y], 1);
        msum.add(mp[y], -y);
      } else {
        pnum.add(mp[y], 1);
        psum.add(mp[y], y);
      }
    }
  }
  cout << ans << endl;
  return 0;
}
