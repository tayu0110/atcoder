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
const ll MOD = 1e9 + 7;
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
  mint comb(mint &n, mint &k, vector<mint> &v) {
    if(n.val < k.val) return 0;
    v.assign(n.val+1, 0);
    vector<mint> d(n.val+1);
    d[0] = 1;
    d[1] = 1;
    for(ll i=2;i<n.val+1;i++) d[i] = d[i-1] * i;
    for(ll i=0;i<n.val+1;i++) {
      if(i-k.val < 0) v[i] = 0;
      else v[i] = d[i] / (d[k.val] * d[i-k.val]);
    }
    return v[n.val];
  }
};

mint comb(mint &n, mint &k, vector<mint> &v) {
  if(n.val < k.val) return 0;
  v.assign(n.val+1, 0);
  vector<mint> d(n.val+1);
  d[0] = 1;
  d[1] = 1;
  for(ll i=2;i<n.val+1;i++) d[i] = d[i-1] * i;
  for(ll i=0;i<n.val+1;i++) {
    if(i-k.val < 0) v[i] = 0;
    else v[i] = d[i] / (d[k.val] * d[i-k.val]);
  }
  return v[n.val];
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, k;
  cin >> n >> k;
  if(n > k) {
    mint nn = n+k;
    mint nk = k;
    vector<mint> v(nn.val+1);
    comb(nn, nk, v);
    cout << v[nn.val-1] << endl;
    return 0;
  }
  k %= n;
  if(k == 0) {
    cout << 1 << endl;
    return 0;
  }
  mint nn, nk;
  nn = n;
  nk = k;
  vector<mint> v;
  mint t;
  mint ans = t.comb(nn, nk, v);
  cout << ans << endl;
  return 0;
}
