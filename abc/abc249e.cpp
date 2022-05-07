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

ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
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
  mint pow(ll t) const {if(!t) return 1; mint a = pow(t >> 1); return (t & 1) ? a * a * (*this) : a * a;}
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
    fact[0] = 1LL; for(ll i = 1; i <= n; i++) fact[i] = fact[i-1] * i;
    ifact[n] = fact[n].inv(); for(ll i = n; i >= 1; i--) ifact[i-1] = ifact[i] * i;
  }
  // {combination c(n); mint ans = c(n, k);} => (ans == nCk)
  mint operator()(int n, int k) { return (k < 0 || k > n) ? 0 : fact[n] * ifact[k] * ifact[n-k]; }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, p;
  cin >> n >> p;
  MOD = p;
  vector<vector<mint>> dp(n+1, vector<mint>(2*n+1, 0)), sum(n+1, vector<mint>(2*n+1, 0));
  dp[0][0] = mint(26) * mint(25).inv();
  for(int i=1;i<n+1;i++) sum[i][0] = dp[0][0];
  for(int j=1;j<2*n;j++) {
    for(int i=1;i<n+1;i++) {
      // for(int k=1;k<n+1;k++) {
      int ten = 1;
      for(int t=1;t<5;t++) {
        // if(i+k > n) break;
        // int t = to_string(k).length() + 1;
        if(j-t-1 < 0) continue;
        int mx = max(i - ten+1, 0);
        int mn = max(i - ten*10+1, 0);
        dp[i][j] += (sum[mx][j-t-1] - sum[mn][j-t-1]) * 25;
        ten *= 10;
        // if(dp[i-k][j-t] == 0) continue;
        // if(i-k > 0) dp[i][j] += dp[i-k][j-t] * 25;
        // else dp[i][j] += dp[i-k][j-t] * 26;
      }
      if(i < n) sum[i+1][j] = sum[i][j] + dp[i][j];
    }
  }
  mint ans = 0;
  for(int i=1;i<n;i++) ans += dp[n][i];
  cout << ans << endl;
  return 0;
}
