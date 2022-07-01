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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n;
  ll x;
  cin >> n >> x;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll mx = *max_element(a.begin(), a.end());
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    ll s = a[i];
    ll xsum = 0;
    while(s*2 <= mx) {
      s <<= 1;
      xsum = xsum*2 + x;
    }
    ll l = 0, r = 0;
    if(mx-s <= xsum) {
      l = mx;
      r = mx;
    } else {
      l = s + xsum;
      r = s*2;
    }
    p[i] = {l, r};
  }
  sort(p.begin(), p.end());
  ll ans = INF;
  for(auto [l, r] : p) {
    ans = min(ans, mx - l);
    mx = max(mx, r);
  }
  if(ans < x) ans = 0;
  cout << ans << endl;
  return 0;
}
