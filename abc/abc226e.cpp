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

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

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
struct UnionFind {
  vector<int> par;
  UnionFind(int n) : par(vector<int>(n, -1)) {}
  int root(int x) {
    if(par[x] < 0) return x;
    return par[x] = root(par[x]);
  }
  bool merge(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    if(rx == ry) return false;
    if(par[rx] > par[ry]) swap(rx, ry);
    par[rx] += par[ry];
    par[ry] = rx;
    return true;
  }
  bool isSame(int x, int y) {
    int rx = root(x);
    int ry = root(y);
    return rx == ry;
  }
  int size(int x) {
    return -par[root(x)];
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<set<int>> t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    t[u].insert(v);
    t[v].insert(u);
  }
  if(n == 2) {
    cout << 0 << endl;
    return 0;
  }
  if(n != m) {
    cout << 0 << endl;
    return 0;
  }
  queue<int> nt;
  for(int i=0;i<n;i++) {
    if(t[i].empty()) {
      cout << 0 << endl;
      return 0;
    }
    if(t[i].size() == 1) nt.push(i);
  }
  while(!nt.empty()) {
    int now = nt.front();
    nt.pop();
    if(t[now].empty()) {
      cout << 0 << endl;
      return 0;
    }
    int op = *t[now].begin();
    t[now].erase(op);
    t[op].erase(now);
    if(t[op].empty()) {
      cout << 0 << endl;
      return 0;
    } else if(t[op].size() == 1) {
      nt.push(op);
    }
  }
  UnionFind uf(n);
  for(int i=0;i<n;i++) {
    for(auto e : t[i]) {
      uf.merge(i, e);
    }
  }
  set<int> ck;
  int k = 0;
  for(int i=0;i<n;i++) {
    int root = uf.root(i);
    if(ck.find(root) != ck.end()) continue;
    ck.insert(root);
    if(uf.size(root) == 1) continue;
    else if(uf.size(root) == 2) {
      cout << 0 << endl;
      return 0;
    } else k++;
  }
  cout << mint(2).pow(k) << endl;
  return 0;
}
