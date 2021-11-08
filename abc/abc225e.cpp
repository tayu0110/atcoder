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
const ll inf = 1LL << 29;
const ld PI = 3.141592653589793238462643383;
template <typename T, T infinity>
struct Rational {
  T num;     // Numerator
  T den;  // Denominator
  Rational() : num{0}, den{1} {}
  Rational(T n, T d) : num{n}, den{d} {
    if(!num) {
      den = 1;
    } else if(!den) {
      num = infinity; den = 0;
    } else {
      T g = gcd(num, den);
      num /= g; den /= g;
    }
  }
  Rational(const Rational &r) : num{r.num}, den{r.den} {}
  bool operator==(const Rational &rhs) const { return num * rhs.den == rhs.num * den; }
  bool operator<(const Rational &rhs) const { return num * rhs.den < rhs.num * den; }
  bool operator>(const Rational &rhs) const { return num * rhs.den > rhs.num & den; }
  bool operator<=(const Rational &rhs) const { return (*this < rhs || *this == rhs); }
  bool operator>=(const Rational &rhs) const { return (*this > rhs || *this == rhs); }
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  using T = Rational<ll, inf>;
  vector<pair<T, T>> t(n);
  for(int i=0;i<n;i++) {
    auto [x, y] = p[i];
    t[i] = {T(y-1, x), T(y, x-1)};
  }
  sort(t.begin(), t.end(), [](pair<T, T> lhs, pair<T, T> rhs) {
    return (lhs.second == rhs.second ? lhs.first < rhs.first : lhs.second < rhs.second);
  });
  int ans = 1;
  T prev = t[0].second;
  for(int i=1;i<n;i++) {
    auto [f, s] = t[i];
    if(prev <= f) {
      ans++;
      prev = s;
    }
  }
  cout << ans << endl;
  return 0;
}
