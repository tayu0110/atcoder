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

ll MOD = 998244353;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
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
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  ll p;
  cin >> n >> p;
  MOD = p;
  vector<vector<vector<mint>>> dp(n, vector<vector<mint>>(n+3, vector<mint>(2, 0)));
  dp[0][0][1] = dp[0][1][0] = 1;
  for(int i=1;i<n;i++) {
    for(int j=0;j<n;j++) {
      dp[i][j+1][0] += dp[i-1][j][0];
      dp[i][j][1] += dp[i-1][j][0] + dp[i-1][j][1];
      dp[i][j+1][1] += dp[i-1][j][1] * 3;
      dp[i][j+2][0] += dp[i-1][j][1] * 2;
    }
  }
  for(int i=1;i<n;i++) {
    cout << dp[n-1][i][1] << endl;
  }
  return 0;
}
