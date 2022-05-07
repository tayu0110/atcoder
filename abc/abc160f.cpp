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

const ll MOD = 1e9 + 7;
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
  constexpr bool operator==(const mint &m) const {return val == m.val;}
  constexpr bool operator<(const mint &m) const {return val < m.val;}
  constexpr bool operator>(const mint &m) const {return val > m.val;}
  constexpr bool operator<=(const mint &m) const {return val <= m.val;}
  constexpr bool operator>=(const mint &m) const {return val >= m.val;}
  constexpr bool operator!=(const mint &m) const {return val != m.val;}
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
int n;
vector<vector<tuple<int, int, mint>>> memo;
pair<int, mint> dfs(int now, int par, vector<vector<int>> &t, combination &c) {
  int rem = 0;
  vector<pair<int, mint>> res;
  for(auto to : t[now]) {
    if(to == par) continue;
    auto r = dfs(to, now, t, c);
    res.push_back(r);
    rem += r.first;
    memo[now].push_back({to, r.first, r.second});
  }
  int res_num = rem;
  mint res_p = 1;
  for(auto [num, p] : res) {
    res_p *= c(rem, num) * p;
    rem -= num;
  }
  return make_pair(res_num+1, res_p);
}
vector<mint> ans;
void dfs(int now, int par, int pnum, mint pp, combination &c) {
  if(par >= 0) {
    memo[now].push_back({par, pnum, pp});
  }
  int nnum = 0;
  mint np = 1;
  for(auto [to, num, p] : memo[now]) {
    nnum += num;
    np *= c(nnum, num) * p;
  }
  ans[now] = np;
  for(auto [to, num, p] : memo[now]) {
    if(to == par) continue;
    dfs(to, now, nnum - num + 1, np / c(nnum, num) / p, c);
  }
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  cin >> n;
  vector<vector<int>> t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--; b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  combination c(n);
  memo.resize(n);
  dfs(0, -1, t, c);
  ans.resize(n);
  dfs(0, -1, -1, 0, c);
  for(auto e : ans) cout << e << endl;
  return 0;
}
