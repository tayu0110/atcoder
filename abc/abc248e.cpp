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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
struct Rational {
  ll numerator, denominator;
  constexpr Rational() : numerator(0), denominator(1) {}
  constexpr Rational(ll val) : numerator(val), denominator(1) {}
  // Rational(num, den) -> num / den
  constexpr Rational(ll num, ll den) : numerator(num), denominator(den) {
    if(!denominator)  { numerator = 1; denominator = 0; return; }
    if(!numerator)    { numerator = 0; denominator = 1; return; }
    ll g = gcd(numerator < 0 ? -numerator : numerator, denominator < 0 ? -denominator : denominator);
    numerator /= g; denominator /= g;
    if(den < 0) { numerator = -numerator; denominator = -denominator; }
  }
  constexpr Rational(const Rational &ratio) : numerator(ratio.numerator), denominator(ratio.denominator) {}
  constexpr bool operator==(const Rational &rhs) const noexcept { return numerator == rhs.numerator && denominator == rhs.denominator; }
  constexpr bool operator<(const Rational &rhs) const noexcept { return numerator * rhs.denominator < rhs.numerator * denominator; }
  constexpr bool operator>(const Rational &rhs) const noexcept { return numerator * rhs.denominator > rhs.numerator * denominator; }
  constexpr bool operator<=(const Rational &rhs) const noexcept { return numerator * rhs.denominator <= rhs.numerator * denominator; }
  constexpr bool operator>=(const Rational &rhs) const noexcept { return numerator * rhs.denominator >= rhs.numerator * denominator; }
  constexpr Rational operator-() const { return Rational(-numerator, denominator); }
  constexpr Rational &operator+=(const Rational &rhs) noexcept {
    numerator = numerator * rhs.denominator + rhs.numerator * denominator;
    denominator *= rhs.denominator;
    ll g = gcd(numerator, denominator);
    numerator /= g, denominator /= g;
    return *this;
  }
  constexpr Rational &operator-=(const Rational &rhs) noexcept { return *this += -rhs; }
  constexpr Rational &operator*=(const Rational &rhs) noexcept {
    if(rhs.is_zero()) { numerator = 0; denominator = 1; return *this; }
    if(rhs.is_inf())  { numerator = 1; denominator = 0; return *this; }
    numerator *= rhs.numerator;
    denominator *= rhs.denominator;
    ll g = gcd(numerator, denominator);
    numerator /= g, denominator /= g;
    return *this;
  }
  constexpr Rational &operator/=(const Rational &rhs) noexcept { return *this *= Rational(rhs.denominator, rhs.numerator); }
  constexpr Rational operator+(const Rational &rhs) const noexcept { return Rational(*this) += rhs; }
  constexpr Rational operator-(const Rational &rhs) const noexcept { return Rational(*this) -= rhs; }
  constexpr Rational operator*(const Rational &rhs) const noexcept { return Rational(*this) *= rhs; }
  constexpr Rational operator/(const Rational &rhs) const noexcept { return Rational(*this) /= rhs; }
  constexpr inline bool is_zero() const { return numerator == 0; }
  constexpr inline bool is_inf() const { return denominator == 0; }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    int x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  if(k == 1) {
    cout << "Infinity" << endl;
    return 0;
  }
  auto f = [](ll x, ll y, ll s, ll t) {
    Rational a(y-t, x-s);
    if(a.is_inf()) return make_pair(a, Rational(x));
    if(a.is_zero()) return make_pair(a, Rational(y));
    Rational b = -a * x + y;
    return make_pair(a, b);
  };
  map<pair<Rational, Rational>, set<int>> mp;
  for(int i=0;i<n;i++) {
    auto [x1, y1] = p[i];
    for(int j=i+1;j<n;j++) {
      auto [x2, y2] = p[j];
      auto now = f(x1, y1, x2, y2);
      mp[now].insert(i);
      mp[now].insert(j);
    }
  }
  int ans = 0;
  for(auto [f, s] : mp) {
    if(s.size() >= k) ans++;
  }
  cout << ans << endl;
  return 0;
}
