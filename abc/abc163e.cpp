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
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    p[i] = {a[i], i};
  }
  sort(p.begin(), p.end(), greater<pll>());
  vector<vector<ll>> dp(n+1, vector<ll>(n+1, -INF));
  dp[0][0] = 0;
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(i+j >= n) continue;
      auto [b, idx] = p[i+j];
      dp[i+1][j] = max(dp[i+1][j], dp[i][j] + b * abs(idx-i));
      dp[i][j+1] = max(dp[i][j+1], dp[i][j] + b * abs(n-1-j-idx));
    }
  }
  ll mx = 0;
  for(int i=0;i<n;i++) {
    mx = max(mx, dp[i][n-i]);
    mx = max(mx, dp[n-i][i]);
  }
  cout << mx << endl;
  return 0;
}
