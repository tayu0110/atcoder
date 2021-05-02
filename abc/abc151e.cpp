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
  mint(ll val=0) : val((val%MOD + MOD) % MOD) {}
  mint(const mint &m) : val(m.val) {}
  mint operator-() const {return mint(-val);}
  mint operator+(const mint &m) {return mint(*this) += m;}
  mint operator-(const mint &m) {return mint(*this) -= m;}
  mint operator*(const mint &m) {return mint(*this) *= m;}
  mint operator/(const mint &m) {return mint(*this) /= m;}
  mint &operator+=(const mint &a) {if((val += a.val) >= MOD) val -= MOD; return *this;}
  mint &operator-=(const mint &a) {if((val -= a.val) < 0) val += MOD; return *this;}
  mint &operator*=(const mint &a) {val = val * a.val % MOD; return *this;}
  mint &operator/=(const mint &m) {
    ll a = m.val, b = MOD, u = 1, v = 0;
    while(b) {
      ll t = a / b;
      a -= t * b; swap(a, b);
      u -= t * v; swap(u, v);
    }
    val = val * u % MOD;
    if(val < 0) val += MOD;
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
void comb(mint &n, mint &k, vector<mint> &v) {
  if(n.val < k.val) return;
  vector<mint> d(n.val);
  d[0] = 1;
  d[1] = 1;
  for(ll i=2;i<n.val;i++) d[i] = d[i-1] * i;
  for(ll i=0;i<n.val;i++) {
    if(i-k.val < 0) v[i] = 0;
    else v[i] = d[i] / (d[k.val] * d[i-k.val]);
  }
  return;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  mint n, k;
  cin >> n >> k;
  k.val--;
  vector<ll> a(n.val);
  for(int i=0;i<n.val;i++) {
    cin >> a[i];
  }
  if(k.val == 0) {
    cout << 0 << endl;
    return 0;
  }
  vector<mint> c(n.val, 1);
  comb(n, k, c);
  sort(a.begin(), a.end(), greater<ll>());
  mint ans = 0;
  int m = n.val-1;
  int l = k.val;
  for(int i=0;m>=l;i++) {
    ans += c[m]*a[i];
    ans -= c[m]*a[m];
    m--;
  }
  cout << ans << endl;
  return 0;
}
