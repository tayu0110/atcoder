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
  ll k;
  cin >> n >> k;
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  ll l = 0, r = a[n-1] * b[n-1];
  while(r - l > 1) {
    ll m = (r + l) / 2;
    ll mn = 0, mx = 0;
    // DEBUG(l);DEBUG_EN(r);
    for(int i=0;i<n;i++) {
      ll t = a[i];
      auto nit = lower_bound(b.begin(), b.end(), m / t);
      auto xit = upper_bound(b.begin(), b.end(), m / t);
      mn += nit - b.begin();
      mx += xit - b.begin();
    }
    // DEBUG(mn);DEBUG_EN(mx);
    if(mn >= k) r = m;
    else if(mx < k) l = m;
    else r = m;
  }
  cout << r << endl;
  return 0;
}
