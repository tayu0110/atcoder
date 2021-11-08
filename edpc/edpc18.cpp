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
template<typename T>
struct Matrix {
  int row;
  int column;
  vector<vector<T>> t;
  Matrix() : row{0}, column{0}, t{vector<vector<T>>(0, vector<T>(0, 0))} {}
  Matrix(int r, int c) : row{r}, column{c}, t{vector<vector<T>>(r, vector<T>(c, 0))} {}
  Matrix(vector<vector<T>> v) : row{(int)v.size()}, column{(int)v[0].size()}, t{v} {}
  Matrix(const Matrix &m) : row{m.row}, column{m.column}, t{m.t} {}
  constexpr Matrix operator+(const Matrix &m) {
    assert(row == m.row && column == m.column);
    Matrix res(*this);
    for(int i=0;i<row;i++) for(int j=0;j<column;j++) res[i][j] = t[i][j] + m[i][j];
    return res;
  }
  constexpr Matrix operator-(const Matrix &m) {
    assert(row == m.row && column == m.column);
    Matrix res(*this);
    for(int i=0;i<row;i++) for(int j=0;j<column;j++) res[i][j] = t[i][j] - m[i][j];
    return res;
  }
  constexpr Matrix operator*(const Matrix &m) {
    assert(column == m.row);
    Matrix res(row, m.column);
    for(int i=0;i<row;i++) for(int j=0;j<m.column;j++) for(int k=0;k<m.row;k++) res.t[i][j] += t[i][k] * m.t[k][j];
    return res;
  }
  constexpr Matrix &operator+=(const Matrix &m) {return *this = *this + m;}
  constexpr Matrix &operator-=(const Matrix &m) {return *this = *this - m;}
  constexpr Matrix &operator*=(const Matrix &m) {return *this = *this * m;}
  void set(int r, int c, T val) {t[r][c] = val;}
  T get(int r, int c) {return t[r][c];}
  Matrix pow(ll p) {
    assert(p > 0);
    if(p == 1) return *this;
    Matrix res = this->pow(p/2);
    res *= res;
    if(p % 2) res *= *this;
    return res;
  }
private:
  Matrix e(int sz) {
    Matrix res(sz, sz);
    for(int i=0;i<sz;i++) res.t[i][i] = 1;
  }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll k;
  cin >> n >> k;
  vector<vector<mint>> a(n, vector<mint>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) cin >> a[i][j];
  Matrix<mint> m(a);
  Matrix<mint> b(n, 1);
  for(int i=0;i<n;i++) b.set(i, 0, 1);
  Matrix res = m.pow(k) * b;
  mint ans = 0;
  for(int i=0;i<n;i++) ans += res.get(i, 0);
  cout << ans << endl;
  return 0;
}
