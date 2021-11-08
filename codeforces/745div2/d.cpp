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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
struct mint {
  ll val;
   mint(ll val=0) : val((val%MOD + MOD) % MOD) {}
  constexpr mint(const mint &m) : val(m.val) {}
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
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, m, k, p;
  cin >> n >> m >> k >> p;
  MOD = p;
  mint ans = 0;
  for(ll i=1;i<=n;i++) {
    if(i > n-m) continue;
    vector<vector<mint>> dp(n, vector<mint>(n+1, 0));
    dp[0][i] = 1;
    const int mn = n-m+1;
    for(int j=0;j<n-1;j++) {
      for(int l=i;l<n;l++) {
        if(i < l && l < n-m+1) continue;
        if(l-j > 0) dp[j+1][l] += dp[j][l] * (l-j);
        if(l == i) {
          dp[j+1][mn] += dp[j][l];
        } else {
          dp[j+1][l+1] += dp[j][l];
        }
      }
      ans += dp[n-1][n] * n;
    }
  }
  cout << ans << endl;
  return 0;
}
