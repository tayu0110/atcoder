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
  constexpr mint &operator/=(const mint m) noexcept {return *this *= m.inv();}
  constexpr mint pow(ll t) const {
    if(!t) return 1;
    mint a = pow(t >> 1);
    a *= a;
    if(t & 1) a *= (*this);
    return a;
  }
  constexpr mint inv() const {return pow(MOD-2);}
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
    fact[0] = 1;
    for(int i = 1; i <= n; i++) fact[i] = fact[i-1] * i;
    ifact[n] = fact[n].inv();
    for(int i = n; i >= 1; i--) ifact[i-1] = ifact[i] * i;
  }
  // {combination c(n); mint ans = c(n, k);} => (ans == nCk)
  mint operator()(int n, int k) {
    if(k < 0 || k > n) return 0;
    return fact[n] * ifact[k] * ifact[n-k];
  }
};
struct LazySegmentTree {
 private:
  int n;
  vector<pair<mint, mint>> node;
  vector<int> lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.resize(2*n, make_pair(mint(0), mint(1))); lazy.resize(2*n, -1);
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  pair<mint, mint> merge(pair<mint, mint> l, pair<mint, mint> r) {
    return {l.first*r.second+r.first, l.second*r.second};
  }
  void setLazy(int k, int v) {
    lazy[k] = v;
  }
  void eval(int now, int l, int r) {
    if(lazy[now] != -1) {
      node[now] = make_pair(mint(lazy[now]), mint(10));
      int k = now;
      while(k < n) {
        k *= 2;
        node[now] = merge(node[now], node[now]);
      }
      if(r - l > 1) setLazy(now*2, lazy[now]), setLazy(now*2+1, lazy[now]);
      lazy[now] = -1;
    }
  }
  void update(int a, int b, ll x, int k=1, int l=0, int r=-1) {
    if(r < 0) r = n;
    eval(k, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      setLazy(k, x);
      eval(k, l, r);
    } else {
      update(a, b, x, 2*k, l, (l+r)/2);
      update(a, b, x, 2*k+1, (l+r)/2, r);
      node[k] = merge(node[2*k], node[2*k+1]);
    }
  }
  pair<mint, mint> get(int a, int b, int now=1, int l=0, int r=-1) {
    if(r < 0) r = n;
    eval(now, l, r);
    if(b <= l || r <= a) return make_pair(mint(0), mint(1));
    if(a <= l && r <= b) return node[now];
    pair<mint, mint> res = get(a, b, 2*now, l, (l+r)/2);
    res = merge(res, get(a, b, 2*now+1, (l+r)/2, r));
    return res;
  }
};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  LazySegmentTree st(n);
  st.update(0, n, 1);
  while(q--) {
    int l, r, d;
    cin >> l >> r >> d;
    l--;
    st.update(l, r, d);
    cout << st.get(0, n).first << endl;
  }
  return 0;
}
