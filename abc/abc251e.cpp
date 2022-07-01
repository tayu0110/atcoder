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
  vector<vector<vector<ll>>> dp(n+1, vector<vector<ll>>(2, vector<ll>(2, INF)));
  dp[0][0][0] = a[n-1];
  dp[0][1][1] = a[0];
  for(int i=1;i<n;i++) {
    for(int j=0;j<2;j++) {
      for(int k=0;k<2;k++) {
        if(j == 0) {
          dp[i][j][k] = min(dp[i][j][k], dp[i-1][1][k]);
        } else {
          ll t = a[i];
          if(k == 0 && i == n-1) t = 0;
          dp[i][j][k] = min(dp[i][j][k], dp[i-1][1][k] + t);
          dp[i][j][k] = min(dp[i][j][k], dp[i-1][0][k] + t);
        }
      }
    }
  }
  cout << min({dp[n-1][0][1], dp[n-1][1][0], dp[n-1][1][1]}) << endl;
  return 0;
}
