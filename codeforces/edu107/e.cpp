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

using namespace std;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
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
  constexpr mint &operator/=(mint m) noexcept {
    ll exp = MOD - 2;
    while(exp) {
      if(exp % 2 != 0) *this *= m;
      m *= m;
      exp /= 2;
    }
    return *this;
  }
  mint pow(ll t) const {
    if(!t) return 1;
    mint a = pow(t >> 1);
    a *= a;
    if(t & 1) a *= (*this);
    return a;
  }
  bool operator==(const mint &m) {return val == m.val;}
  bool operator<(const mint &m) {return val < m.val;}
  bool operator>(const mint &m) {return val > m.val;}
  bool operator<=(const mint &m) {return val <= m.val;}
  bool operator>=(const mint &m) {return val >= m.val;}
  bool operator!=(const mint &m) {return val != m.val;}
  friend ostream &operator<<(ostream &os, const mint &m) {os << m.val; return os;}
  friend istream &operator>>(istream & is, mint &m) {is >> m.val; return is;}
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<string> g(n);
  for(int i=0;i<n;i++) {
    cin >> g[i];
  }
  vector<vector<vector<mint>>> dp(n, vector<vector<mint>>(m, vector<mint>(2)));
  vector<vector<bool>> ck(n, vector<bool>(m, false));
  ck[0][0] = true;
  queue<pii> nt;
  nt.push({0, 0});
  mint ans = 0;
  while(!nt.empty()) {
    int r = nt.front().first;
    int c = nt.front().second;
    nt.pop();
    if(r + 1 < n && !ck[r+1][c]) {
      nt.push({r+1, c});
      ck[r+1][c] = true;
    }
    if(c + 1 < m && !ck[r][c+1]) {
      nt.push({r, c+1});
      ck[r][c+1] = true;
    }
    if(g[r][c] == '*') {
      for(int i=0;i<2;i++) ans += dp[r][c][i];
      continue;
    }
    if(r + 1 < n) {
      if(g[r+1][c] == '*') {
        for(int i=0;i<2;i++) dp[r+1][c][0] += dp[r][c][i];
      } else {
        
      }
    }
    if(c + 1 < m) {
      if(g[r][c+1] == '*') {
        for(int i=0;i<2;i++) dp[r][c+1][0] += dp[r][c][i];
      }
    } 
  }
  for(int i=0;i<2;i++) ans += dp[n-1][m-1][i];
  cout << ans << endl;
  return 0;
}
