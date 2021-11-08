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

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 998244353;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
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
  constexpr mint pow(ll t) const {
    if(!t) return 1;
    mint a = pow(t >> 1);
    a *= a;
    if(t & 1) a *= (*this);
    return a;
  }
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
    fact[0] = 1LL;
    for(ll i = 1; i <= n; i++) fact[i] = fact[i-1] * i;
    ifact[n] = fact[n].inv();
    for(ll i = n; i >= 1; i--) ifact[i-1] = ifact[i] * i;
  }
  // {combination c(n); mint ans = c(n, k);} => (ans == nCk)
  mint operator()(int n, int k) {
    if(k < 0 || k > n) return 0;
    return fact[n] * ifact[k] * ifact[n-k];
  }
};
class FenwickTree {
  vector<mint> t;
  int sz;
public: 
  FenwickTree(int n) : sz(n+1), t(vector<mint>(n+1, 0)) {}
  // fix the 0-based array to the 1-based array
  FenwickTree(vector<mint> &s) : sz(s.size() + 1) {
    t.assign(sz, 0);
    for(int i=0;i<s.size();i++) add(i, s[i]);
  }
  // add(idx, val) => add val to idx th element
  // idx : 0-based index
  void add(int idx, mint val) {
    idx++;
    while(idx < sz) {
      t[idx] += val;
      idx += idx & -idx;
    }
  }
  // getSum(l, r) => get the sum of [l, r)
  // l, r : 0-based index
  mint getSum(int l, int r) {
    // need not fix l and r to 1-based index
    // because subGetSum() needs a 0-based index argument.
    return subGetSum(r) - subGetSum(l);
  }
private:
  // subGetSum(r) => get the sum of [0, r)
  // r : 0-based index
  mint subGetSum(int r) {
    if(r < 0) return 0;
    if(r >= sz) r = sz-1;
    mint res = 0;
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
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  map<ll, int> mp;
  for(int i=0;i<n;i++) mp[a[i]] = 0;
  int cnt = 0;
  for(auto &[f, s] : mp) s = cnt++;
  for(int i=0;i<n;i++) a[i] = mp[a[i]];
  mint ans = 0;
  FenwickTree ft(n);
  mint p = 1;
  for(int i=0;i<n;i++) {
    int now = a[i];
    if(i > 0) ans += ft.getSum(0, now+1) * mint(2).pow(i-1);
    ft.add(now, p);
    p /= 2LL;
  }
  cout << ans << endl;
  return 0;
}
