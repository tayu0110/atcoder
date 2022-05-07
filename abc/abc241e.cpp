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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  ll x = 0;
  vector<pll> p;
  set<int> ck;
  ll nk = k;
  while(nk--) {
    int idx = x % n;
    x += a[idx];
    p.push_back({x, idx});
    if(ck.find(idx) != ck.end()) break;
    ck.insert(idx);
  }
  if(nk < 0) {
    cout << x << endl;
    return 0;
  }
  auto [nx, ni] = p.back();
  for(int i=0;i<p.size();i++) {
    auto [px, pi] = p[i];
    if(ni == pi) {
      ll span_sum = nx - px;
      ll span = p.size() - i - 1;
      ll ans = 0;
      if(i) {
        k -= i;
        ans += p[i-1].first;
      }
      if(!span) {
        ans += k * span_sum;
        cout << ans << endl;
        return 0;
      }
      ans += (k / span) * span_sum;
      k %= span;
      if(!k) {
        cout << ans << endl;
        return 0;
      }
      k--;
      ans += p[i+k].first;
      if(i) ans -= p[i-1].first;
      cout << ans << endl;
      return 0;
    }
  }
  return 0;
}
