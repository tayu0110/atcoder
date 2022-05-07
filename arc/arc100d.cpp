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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n-1;i++) a[i+1] += a[i];
  ll ans = accumulate(a.begin(), a.end(), 0LL);
  for(int i=2;i<n-1;i++) {
    ll f = a[i-1];
    ll b = a[n-1] - a[i-1];
    auto pit = upper_bound(a.begin(), a.begin()+i, f / 2);
    auto bit = upper_bound(a.begin()+i, a.end(), b / 2 + f);
    ll p = -1, q = -1, r = -1, s = -1;
    if(pit != a.begin()+i) {
      p = *pit;
      q = f - p;
    }
    if(bit != a.end()) {
      r = *bit - f;
      s = b - r;
    }
    if(pit != a.begin()) {
      pit--;
      ll tp = *pit, tq = f - tp;
      if(abs(p-q) > abs(tp-tq)) {
        p = tp; q = tq;
      }
    }
    if(bit != a.begin()+i) {
      bit--;
      ll tr = *bit - f, ts = b - tr;
      if(abs(r-s) > abs(tr-ts)) {
        r = tr; s = ts;
      }
    }
    ll mx = max({p, q, r, s});
    ll mn = min({p, q, r, s});
    if(mn < 0) continue;
    ans = min(ans, mx - mn);
  }
  cout << ans << endl;
  return 0;
}
