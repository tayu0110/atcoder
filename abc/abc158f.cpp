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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 998244353;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
struct mint {
  ll val;
  constexpr mint(ll val=0) : val((val%MOD + MOD) % MOD) {}
  constexpr mint(const mint &m) : val(m.val) {}
  constexpr mint operator-() const {return mint(-val);}
  constexpr mint operator+(const mint &m) const noexcept {return mint(*this) += m;}
  constexpr mint operator-(const mint &m) const noexcept {return mint(*this) -= m;}
  constexpr mint operator*(const mint &m) const noexcept {return mint(*this) *= m;}
  constexpr mint operator/(const mint &m) const noexcept {return mint(*this) /= m;}
  constexpr mint &operator+=(const mint &a) noexcept {if((val += a.val) >= MOD) val -= MOD; return *this;}
  constexpr mint &operator-=(const mint &a) noexcept {if((val -= a.val) < 0) val += MOD; return *this;}
  constexpr mint &operator*=(const mint &a) noexcept {val = val * a.val % MOD; return *this;}
  constexpr mint &operator/=(const mint m) noexcept {return *this *= m.inv();}
  constexpr mint pow(ll t) const {if(!t) return 1; mint a = pow(t >> 1); return (t & 1) ? a * a * (*this) : a * a;}
  constexpr mint inv() const {return pow(MOD-2);}
  bool operator==(const mint &m) {return val == m.val;}
  bool operator<(const mint &m) {return val < m.val;}
  bool operator>(const mint &m) {return val > m.val;}
  bool operator<=(const mint &m) {return val <= m.val;}
  bool operator>=(const mint &m) {return val >= m.val;}
  bool operator!=(const mint &m) {return val != m.val;}
  friend ostream &operator<<(ostream &os, const mint &m) {os << m.val; return os;}
  friend istream &operator>>(istream & is, mint &m) {is >> m.val; return is;}
};
struct combination {
  vector<mint> fact, ifact;
  combination(int n) : fact(n+1), ifact(n+1) {
    assert(n < MOD);
    fact[0] = 1LL; for(ll i = 1; i <= n; i++) fact[i] = fact[i-1] * i;
    ifact[n] = fact[n].inv(); for(ll i = n; i >= 1; i--) ifact[i-1] = ifact[i] * i;
  }
  // {combination c(n); mint ans = c(n, k);} => (ans == nCk)
  mint operator()(int n, int k) { return (k < 0 || k > n) ? 0 : fact[n] * ifact[k] * ifact[n-k]; }
};
template<class T>
struct SegmentTree {
 private:
  using update_func_t = function<T(T element, T val)>;
  int sz;
  T def_val;
  vector<T> tree;
  update_func_t update_func;
 public:
  SegmentTree(int size, T default_val, update_func_t update_func) : def_val(default_val), update_func(update_func) {
    sz = 1; while(sz < size) sz <<= 1;
    tree.assign(sz*2, def_val);
  }
  void update(int idx, T val) {
    idx += sz;
    tree[idx] = val;
    while(idx >> 1) {
      idx >>= 1;
      tree[idx] = update_func(tree[idx*2], tree[idx*2+1]);
    }
  }
  T get(int l, int r) {
    return do_get(l, r, 1, 0, sz);
  }
 private:
  T do_get(int l, int r, int now, int a, int b) {
    if(r <= a || b <= l) return def_val;
    if(l <= a && b <= r) return tree[now];
    T res = def_val;
    res = update_func(res, do_get(l, r, now*2,   a, (a+b)/2));
    res = update_func(res, do_get(l, r, now*2+1, (a+b)/2, b));
    return res;
  }
};
struct RangeMinimumQuery : SegmentTree<ll> { RangeMinimumQuery(int size) : SegmentTree(size, INF, [](ll lhs, ll rhs) { return min(lhs, rhs); }) {} };
struct RangeMaximumQuery : SegmentTree<ll> { RangeMaximumQuery(int size) : SegmentTree(size, -INF, [](ll lhs, ll rhs) { return max(lhs, rhs); }) {} };
struct RangeSumQuery : SegmentTree<ll> { RangeSumQuery(int size) : SegmentTree(size, 0, [](ll lhs, ll rhs) { return lhs + rhs; }) {} };
struct RMinRSumQuery : SegmentTree<pll> { RMinRSumQuery(int size) : SegmentTree(size, make_pair(0LL, 0LL), [](pll lhs, pll rhs) { return make_pair(min(lhs.first, lhs.second+rhs.first), lhs.second+rhs.second); }) {} };
int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    int x, d;
    cin >> x >> d;
    p[i] = {x, d};
  }
  sort(p.begin(), p.end());
  RangeMaximumQuery r(n);
  vector<ll> x(n);
  set<ll> ck;
  for(int i=0;i<n;i++) {
    auto [nx, nd] = p[i];
    r.update(i, nx+nd);
    x[i] = nx;
    ck.insert(nx);
  }
  for(int i=n-2;i>=0;i--) {
    ll mx = r.get(i, i+1);
    int idx = lower_bound(x.begin(), x.end(), mx) - x.begin();
    if(idx <= i) continue;
    ll nmx = r.get(i, idx);
    r.update(i, nmx);
  }
  map<ll, int> mp, rmp;
  for(int i=0;i<n;i++) {
    mp[x[i]]++;
    mp[r.get(i, i+1)]++;
  }
  int cnt = 0;
  for(auto &[f, s] : mp) s = cnt++, rmp[s] = f;
  vector<mint> dp(cnt+1);
  dp[0] = 1;
  for(int i=0;i<cnt;i++) {
    ll f = rmp[i];
    dp[i+1] += dp[i];
    if(ck.find(f) != ck.end()) {
      int idx = lower_bound(x.begin(), x.end(), f) - x.begin();
      ll next = r.get(idx, idx+1);
      dp[mp[next]] += dp[i];
    }
  }
  cout << dp[cnt] << endl;
  return 0;
}
