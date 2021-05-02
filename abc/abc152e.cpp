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
  constexpr bool operator==(const mint &m) {return val == m.val;}
  constexpr bool operator<(const mint &m) {return val < m.val;}
  constexpr bool operator>(const mint &m) {return val > m.val;}
  constexpr bool operator<=(const mint &m) {return val <= m.val;}
  constexpr bool operator>=(const mint &m) {return val >= m.val;}
  constexpr bool operator!=(const mint &m) {return val != m.val;}
  friend ostream &operator<<(ostream &os, const mint &m) {os << m.val; return os;}
  friend istream &operator>>(istream & is, mint &m) {is >> m.val; return is;}
};
ll modPow(long long a, long long n, long long p) {
  if (n == 0) return 1;
  if (n == 1) return a % p;
  if (n % 2 == 1) return (a * modPow(a, n - 1, p)) % p;
  long long t = modPow(a, n / 2, p);
  return (t * t) % p;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> p(1000005);
  vector<bool> c(1000005, false);
  set<ll> ck;
  for(int i=2;i<1000005;i++) {
    if(c[i]) continue;
    for(int j=2;i*j<1000005;j++) c[i*j] = true;
    ck.insert(i);
  }
  map<ll, ll> mp;
  ll mx = 0;
  for(int i=0;i<n;i++) {
    ll s;
    cin >> s;
    mx = max(mx, s);
    mp[s]++;
    if(mp[s] > 1) continue;
    for(int i=2;i*i<=s;i++) {
      ll k = 0;
      while(s % i == 0) {
        s /= i;
        k++;
      }
      p[i] = max(p[i], k);
    }
    if(s != 1) p[s] = max(p[s], 1LL);
  }
  ll g = 1;
  for(auto i : ck) {
    if(!p[i]) continue;
    g *= modPow(i, p[i], MOD);
    g %= MOD;
  }
  mint ans = 0;
  for(auto e : mp) {
    for(int i=0;i<e.second;i++) {
      ans += mint(1) / e.first;
    }
  }
  ans *= g;
  cout << ans << endl;
  return 0;
}
