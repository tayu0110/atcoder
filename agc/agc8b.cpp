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
  int n, k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<ll> b(n+1), c(n+1);
  for(int i=1;i<n+1;i++) {
    b[i] = b[i-1] + a[i-1];
    c[i] = c[i-1] + max(a[i-1], 0LL);
  }
  ll ans = 0;
  for(int i=0;i<n+1;i++) {
    if(i + k > n) break;
    // DEBUG(i);
    ll tmp = max(0LL, b[i+k] - b[i]);
    // DEBUG(tmp);
    tmp += (i > 0 ? c[i] : 0LL) + (i+k < n ? c[n] - c[i+k] : 0);
    ans = max(ans, tmp);
    // DEBUG_EN(ans);
  }
  cout << ans << endl;
  return 0;
}
