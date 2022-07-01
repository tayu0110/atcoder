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
  int n, k;
  cin >> n >> k;
  vector<int> h(n+1);
  for(int i=1;i<n+1;i++) cin >> h[i];
  if(n == k) {
    cout << 0 << endl;
    return 0;
  }
  if(k == 0) {
    ll ans = 0;
    for(int i=0;i<n;i++) ans += max(0, h[i+1] - h[i]);
    cout << ans << endl;
    return 0;
  }
  vector<vector<ll>> dp(n+1, vector<ll>(n+1, INF));
  dp[0][0] = 0;
  for(int i=1;i<n+1;i++) {
    for(int j=1;j<n+1;j++) {
      for(int k=0;k<i;k++) {
        dp[i][j] = min(dp[i][j], dp[k][j-1] + max(0, h[i] - h[k]));
      }
    }
  }
  ll ans = INF;
  for(int i=1;i<n+1;i++) ans = min(ans, dp[i][n-k]);
  cout << ans << endl;
  return 0;
}
