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
// d : gcd(a, b)
ll extGCD(ll a, ll b, ll &p, ll &q) {
  if(b == 0) {
    p = 1; q = 0;
    return a;
  }
  ll d = extGCD(b, a%b, q, p);
  q -= a/b * p;
  return d;
}
// Return the value of b1 for mod m1 and b2 for mod m2 when m1 and m2 are coprime
// If the return value are (r, m), x = r (mod m), m = lcm(m1, m2). 
// If the return value are (0, -1), there is no solution.
pll crt(ll b1, ll m1, ll b2, ll m2) {
  ll p, q;
  ll d = extGCD(m1, m2, p, q);
  if((b2 - b1) % d != 0) return make_pair(0, -1);
  ll m = m1 * (m2/d);
  ll tmp = (b2 - b1) / d * p % (m2/d);
  auto mod = [](ll a, ll m) { return (a%m + m) % m; };
  ll r = mod(b1 + m1 * tmp, m);
  return make_pair(r, m);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n;
  cin >> n;
  vector<ll> t;
  for(ll i=1;i*i<=2*n;i++) {
    if((2 * n) % i) continue;
    t.push_back(i);
    if(i*i != 2 * n) t.push_back(2 * n / i);
  }
  ll ans = INF;
  for(int i=0;i<t.size();i++) {
    ll now = t[i];
    auto [r, m] = crt(0, now, -1, 2*n / now);
    if(!r) continue;
    ans = min(ans, r);
  }
  cout << ans << endl;
  return 0;
}
