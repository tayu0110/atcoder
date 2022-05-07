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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<ll> osum(n), esum(n);
  for(int i=0;i<n;i++) {
    if(i) {
      osum[i] += osum[i-1];
      esum[i] += esum[i-1];
    }
    if(i % 2) osum[i] += a[i];
    else esum[i] += a[i];
  }
  while(q--) {
    ll x;
    cin >> x;
    auto it = lower_bound(a.begin(), a.end(), x);
    if(it == a.end()) {
      if(n % 2) cout << esum[n-1] << endl;
      else cout << osum[n-1] << endl;
      continue;
    }
    int pos = it - a.begin();
    if(pos && a[pos] - x >= x - a[pos-1]) pos--;
    int l = 1, r = n;
    while(r - l > 1) {
      int m = (r + l) / 2;
      if(n - m <= pos) {
        r = m;
        continue;
      }
      int nr = n - m - 1;
      int nl = nr - m + 1;
      if(nl < 0) {
        r = m;
        continue;
      }
      if(nl > pos) {
        l = m;
        continue;
      }
      ll ldiff = x - a[nl];
      ll rdiff = a[nr] - x;
      if(ldiff < rdiff) {
        if(!nl) {
          l = m;
          continue;
        }
        ll lldiff = x - a[nl-1];
      } else {

      }
    }
  }
  return 0;
}
