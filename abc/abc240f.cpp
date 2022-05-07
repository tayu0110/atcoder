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
ll search(ll b, ll x, ll y) {
  ll l = 1, r = y;
  auto f = [x, b](ll m) { return m * (m-1) / 2 * x + m * b; };
  while(r-l > 2) {
    ll m1 = (2*l+r) / 3;
    ll m2 = (l+2*r) / 3;
    ll sum1 = f(m1);
    ll sum2 = f(m2);
    if(sum1 > sum2) r = m2;
    else l = m1;
  }
  ll mx = -INF;
  ll res = l-5;
  for(ll i=max(l-5, 1LL);i<=min(r+5, y);i++) {
    ll tmp = f(i);
    if(tmp > mx) {
      mx = tmp;
      res = i;
    }
  }
  return res;
}
void hollow(int n, int m, vector<pll> &p) {
  vector<ll> c(m);
  int cnt = 0;
  for(int i=0;i<n;i++) {
    auto [x, y] = p[i];
    for(int j=0;j<y;j++) c[cnt++] = x;
  }
  vector<ll> b(m);
  for(int i=0;i<m;i++) {
    b[i] = c[i];
    if(i) b[i] += b[i-1];
  }
  vector<ll> a(m);
  for(int i=0;i<m;i++) {
    a[i] = b[i];
    if(i) a[i] += a[i-1];
  }
  print_with_space(c);
  print_with_space(b);
  print_with_space(a);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  auto f = [](ll x, ll y, ll b) { return y * (y-1) / 2 * x + y * b; };
  vector<ll> res;
  while(t--) {
    int n, m;
    cin >> n >> m;
    vector<pll> p(n);
    for(int i=0;i<n;i++) {
      int x, y;
      cin >> x >> y;
      p[i] = {x, y};
    }
    ll ans = p[0].first;
    ll b = 0;
    ll sum = 0;
    for(int i=0;i<n;i++) {
      auto [x, y] = p[i];
      if(!x) {
        if(i) ans = max(ans, sum);
        sum += b * y;
        ans = max(ans, sum);
        continue;
      }
      if(b >= 0) {
        if(x > 0) {
          sum += f(x, y, b+x);
          ans = max(ans, sum);
        } else {
          ll idx = search(b+x, x, y);
          ll tmp = f(x, idx, b+x);
          ans = max(ans, sum+tmp);
          sum += f(x, y, b+x);
        }
      } else {
        if(x > 0) {
          ans = max(ans, sum);
          sum += f(x, y, b+x);
          ans = max(ans, sum);
        } else {
          if(i) ans = max(ans, sum);
          else ans = max(ans, sum+x);
          sum += f(x, y, b+x);
        }
      }
      b += x * y;
    }
    res.push_back(ans);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
