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
ll sum_idx(ll sum, vector<int> &a) {
  ll res = 0;
  int n = a.size();
  for(int i=0;i<n;i++) {
    int tar = sum - a[i];
    int idx = lower_bound(a.begin(), a.end(), tar) - a.begin();
    res += n - idx;
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll m;
  cin >> n >> m;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  sort(a.begin(), a.end());
  int l = 2, r = 1001001;
  while(r-l > 1) {
    int mid = (r+l) / 2;
    ll sum = sum_idx(mid, a);
    if(sum >= m) l = mid;
    else r = mid;
  }
  vector<ll> b(n+1);
  for(int i=0;i<n;i++) b[i+1] = a[i] + b[i];
  ll ans = 0;
  for(int i=0;i<n;i++) {
    int tar = l - a[i];
    int idx = lower_bound(a.begin(), a.end(), tar) - a.begin();
    ans += (ll)a[i] * (n-idx) + (b[n] - b[idx]);
  }
  ll over = sum_idx(l, a) - m;
  ans -= over * l;
  cout << ans << endl;
  return 0;
}
