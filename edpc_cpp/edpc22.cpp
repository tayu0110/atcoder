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

ll MOD;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
struct mint {
  ll val;
  mint(ll val=0) : val((val%MOD + MOD) % MOD) {}
  mint(const mint &m) : val(m.val) {}
  mint operator-() const {return mint(-val);}
  mint operator+(const mint &m) const noexcept {return mint(*this) += m;}
  mint operator-(const mint &m) const noexcept {return mint(*this) -= m;}
  mint operator*(const mint &m) const noexcept {return mint(*this) *= m;}
  mint operator/(const mint &m) const noexcept {return mint(*this) /= m;}
  mint &operator+=(const mint &a) noexcept {if((val += a.val) >= MOD) val -= MOD; return *this;}
  mint &operator-=(const mint &a) noexcept {if((val -= a.val) < 0) val += MOD; return *this;}
  mint &operator*=(const mint &a) noexcept {val = val * a.val % MOD; return *this;}
  mint &operator/=(const mint m) noexcept {return *this *= m.inv();}
  mint pow(ll t) const {
    if(!t) return 1;
    mint a = pow(t >> 1);
    a *= a;
    if(t & 1) a *= (*this);
    return a;
  }
  mint inv() const {return pow(MOD-2);}
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
vector<mint> dp;
mint dfs(int now, int par, Graph &t) {
  mint res = 1;
  for(auto e : t[now]) {
    if(e == par) continue;
    res *= dfs(e, now, t) + 1;
  }
  return dp[now] = res;
}
vector<mint> ans;
void rec(int now, int par, Graph &t) {
  ans[now] = 1;
  for(auto e : t[now]) ans[now] *= dp[e] + 1;
  int n = t[now].size();
  vector<mint> l(n), r(n);
  for(int i=0;i<n;i++) l[i] = r[i] = dp[t[now][i]] + 1;
  for(int i=1;i<n;i++) l[i] *= l[i-1];
  for(int i=n-2;i>=0;i--) r[i] *= r[i+1];
  for(int i=0;i<n;i++) {
    if(t[now][i] == par) continue;
    dp[now] = 1;
    if(i) dp[now] *= l[i-1];
    if(i+1 < n) dp[now] *= r[i+1];
    rec(t[now][i], now, t);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll m;
  cin >> n >> m;
  MOD = m;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int x, y;
    cin >> x >> y;
    x--;y--;
    t[x].push_back(y);
    t[y].push_back(x);
  }
  dp.assign(n, 0);
  ans.assign(n, 0);
  dfs(0, -1, t);
  rec(0, -1, t);
  for(int i=0;i<n;i++) cout << ans[i] << endl;
  return 0;
}
