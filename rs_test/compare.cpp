#include <iostream>
#include <vector>
#include <random>
#include <assert.h>
#include <fstream>

using ll = long long;

const ll MOD = 1000000007;

using namespace std;

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
  constexpr mint pow(ll t) const {if(!t) return 1; mint a = pow(t >> 1); return (t & 1) ? a * a * (*this) : a * a;}
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
    fact[0] = 1LL; for(ll i = 1; i <= n; i++) fact[i] = fact[i-1] * i;
    ifact[n] = fact[n].inv(); for(ll i = n; i >= 1; i--) ifact[i-1] = ifact[i] * i;
  }
  // {combination c(n); mint ans = c(n, k);} => (ans == nCk)
  mint operator()(int n, int k) { return (k < 0 || k > n) ? 0 : fact[n] * ifact[k] * ifact[n-k]; }
};
int main() {
  int q = 100000;
  random_device rd;
  default_random_engine eng(rd());
  uniform_int_distribution<ll> distr(1, 100000000000000);

  ofstream ofs("testdata.txt");

  while(q--) {
    mint s = distr(eng);
    mint t = distr(eng);

    ofs << s << " " << t << " " << s + t << " " << s - t << " " << s * t << " " << s / t << " " << s.pow(t.val) << endl;
  }

  ofstream ofs2("testdata2.txt");
  combination c(100000);
  random_device rd2;
  default_random_engine eng2(rd2());
  uniform_int_distribution<int> distr2(1, 100000);

  q = 10000000;
  while(q--) {
    int s = distr2(eng2);
    int t = distr2(eng2);

    ofs2 << s << " " << t << " " << c(max(s, t), min(s, t)) << endl;
  }

  return 0;
}