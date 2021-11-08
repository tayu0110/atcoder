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
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, k;
  cin >> n >> m >> k;
  vector<int> a(m);
  for(int i=0;i<m;i++) cin >> a[i], a[i]--;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  map<pii, ll> mp;
  queue<pii> nt;
  queue<int> nt2;
  for(int i=0;i<m-1;i++) {
    int from = a[i], to = a[i+1];
    vector<int> d(n, -1);
    nt.push({from, 0});
    while(!nt.empty()) {
      auto [now, nd] = nt.front();
      nt.pop();
      if(d[now] >= 0) continue;
      d[now] = nd;
      for(auto e : t[now]) if(d[e] < 0) nt.push({e, nd+1});
    }
    nt2.push(to);
    while(!nt2.empty()) {
      int now = nt2.front();
      nt2.pop();
      for(auto e : t[now]) {
        if(d[now] - d[e] == 1) {
          nt2.push(e);
          mp[{min(e, now), max(e, now)}] += 1;
          break;
        }
      }
    }
  }
  int mx = 0;
  for(auto [f, s] : mp) mx += s;
  vector<mint> dp(mx+1, 0);
  dp[0] = 1;
  for(auto [f, s] : mp) {
    for(int i=mx;i>=0;i--) {
      if(i+s > mx) continue;
      if(dp[i] == mint(0)) continue;
      dp[i+s] += dp[i];
    }
  }
  mint ans = 0;
  for(int i=0;i<=mx;i++) {
    int r = i;
    int b = mx-i;
    if(r-b == k) ans += dp[i];
  }
  ans *= mint(2).pow(n-1 - mp.size());
  cout << ans << endl;
  return 0;
}
