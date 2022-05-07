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
  cin >> n;
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  vector<vector<vector<ll>>> dp(n+1, vector<vector<ll>>(2, vector<ll>(2, INF)));
  dp[0][0][0] = a[0];
  dp[0][1][1] = 0;
  for(int i=0;i<n;i++) {
    dp[i+1][0][0] = min({dp[i+1][0][0], dp[i][0][0] + (i+1 == n ? 0 : a[i+1]) + b[i], dp[i][1][0] + (i+1 == n ? 0 : a[i+1])});
    dp[i+1][0][1] = min({dp[i+1][0][1], dp[i][0][1] + (i+1 == n ? 0 : a[i+1]) + b[i], dp[i][1][1] + (i+1 == n ? 0 : a[i+1])});
    dp[i+1][1][0] = min({dp[i+1][1][0], dp[i][0][0], dp[i][1][0] + b[i]});
    dp[i+1][1][1] = min({dp[i+1][1][1], dp[i][0][1], dp[i][1][1] + b[i]});
  }
  cout << min(dp[n][0][0], dp[n][1][1]) << endl;
  return 0;
}
