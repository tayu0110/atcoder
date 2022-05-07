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
const ld PI = acos(-1);
ll solve(ll k, vector<ll> &a, vector<ll> &b, vector<ll> &c, vector<ll> &d) {
  // DEBUG_EN(k);
  ll l = 0, r = 1e18+10;
  bool f = (a == b);
  while(r - l > 1) {
    ll m = (r + l) / 2;
    ll sum = 0;
    for(int i=0;i<a.size();i++) {
      ll t = m / a[i];
      int pos = upper_bound(b.begin(), b.end(), t) - b.begin();
      if(f) {
        if(!pos) break;
        pos--;
        // DEBUG(i);DEBUG_EN(pos);
        if(pos <= i) break;
        sum += pos - i;
      } else {
        sum += pos;
      }
    }
    if(f) {
      for(int j=0;j<c.size();j++) {
        ll t = m / c[j];
        int pos = upper_bound(d.begin(), d.end(), t) - d.begin();
        if(!pos) break;
        pos--;
        // DEBUG(j);DEBUG_EN(pos);
        if(pos <= j) break;
        sum += pos - j;
      }
    }
    // DEBUG(m);DEBUG_EN(sum);
    if(sum >= k) r = m;
    else l = m;
  }
  return r;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<ll> neg, pos, zero;
  for(int i=0;i<n;i++) {
    if(a[i] < 0) neg.push_back(-a[i]);
    else if(a[i] == 0) zero.push_back(a[i]);
    else pos.push_back(a[i]);
  }
  sort(neg.begin(), neg.end());
  sort(pos.begin(), pos.end());
  ll nn = neg.size();
  ll pn = pos.size();
  if(nn * pn >= k) {
    ll nk = nn * pn - k + 1;
    cout << -solve(nk, neg, pos, pos, neg) << endl;
    return 0;
  }
  ll zn = zero.size();
  ll nk = k - nn * pn - zn * (pn + nn) - zn * (zn-1) / 2;
  DEBUG_EN(nk);
  if(nk <= 0) {
    cout << 0 << endl;
    return 0;
  }
  cout << solve(nk, neg, neg, pos, pos) << endl;
  return 0;
}
