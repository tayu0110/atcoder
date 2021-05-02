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

struct SegmentTree {
  int sz;
  vector<int> t;
  SegmentTree(int n) {
    sz = 1;
    while(sz < n) sz *= 2;
    t.assign(2 * sz - 1, 0);
  }
  void init(int idx, ll val) {
    idx += sz-1;
    t[idx] = val;
    return;
  }
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] > val) break;
      t[idx] = val;
    }
    return;
  }
  void update(int l, int r, ll val, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(r < a || l > b) return;
    if(l <= a && b <= r) {
      t[now] -= val;
      return;
    }
    update(l, r, val, now*2+1, a, (a+b)/2);
    update(l, r, val, now*2+2, (a+b)/2, b);
    return;
  }
  int getMax(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res = max(res, getMax(l, r, 2*now+1, a, (a+b)/2));
    res = max(res, getMax(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
  ll getElement(int idx) {
    if(idx == 0) return t[0];
    ll res = t[idx];
    return res += getElement((idx-1)/2);
  }
  ll operator[](int idx) {
    idx += sz-1;
    return getElement(idx);
  }
};

struct mint {
  ll val;
  constexpr mint(ll val=0) : val((val%MOD + MOD) % MOD) {}
  constexpr mint(const mint &m) : val(m.val) {}
  constexpr mint operator-() const {return mint(-val);}
  constexpr mint operator+(const mint &m) const noexcept {return mint(*this) += m;}
  constexpr mint operator-(const mint &m) const noexcept {return mint(*this) -= m;}
  constexpr mint operator*(const mint &m) const noexcept {return mint(*this) *= m;}
  constexpr mint operator/(const mint &m) const noexcept {return mint(*this) /= m;}
  constexpr mint operator++() const noexcept {return mint(*this) += 1;};
  constexpr mint operator--() const noexcept {return mint(*this) -= 1;};
  constexpr mint operator++(int) const noexcept {return mint(*this) += 1;};
  constexpr mint operator--(int) const noexcept {return mint(*this) -= 1;};
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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  mint k = 0;
  for(int i=0;i<h;i++) {
    cin >> s[i];
    for(int j=0;j<w;j++) if(s[i][j] == '.') k += 1;
  }
  vector<vector<int>> v(h+1, vector<int>(w, 0)), t(h, vector<int>(w+1, 0));
  for(int i=0;i<2;i++) {
    for(int j=0;j<2;j++) {
      if(i == j) continue;
      for(int k=0;k<h;k++) for(int l=0;l<w;l++) {
        if(i && k+i < h+1 && s[k][l] == '.') v[k+i][l] = v[k][l] + 1;
        if(j && l+j < w+1 && s[k][l] == '.') t[k][l+j] = t[k][l] + 1;
      }
    }
  }
  for(int k=h-1;k>=0;k--) for(int l=w-1;l>=0;l--) {
    if(s[k][l] != '.') continue;
    v[k][l] = max(v[k][l], v[k+1][l]);
    t[k][l] = max(t[k][l], t[k][l+1]);
  }
  mint ans = k;
  ans *= mint(2).pow(k.val);
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    if(s[i][j] != '.') continue;
    mint m = k - (v[i+1][j] + t[i][j+1] - 1);
    ans -= mint(2).pow(m.val);
  }
  cout << ans << endl;
  return 0;
}
