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
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<vector<ll>> dp(n+1, vector<ll>(3, -INF));
  dp[0][0] = 0;
  dp[0][1] = a[0];
  dp[1][0] = max(a[0], a[1]);
  dp[1][1] = max(a[0], a[1]);
  for(int i=2;i<n;i++) {
    dp[i][0] = max(dp[i][0], dp[i-2][0] + a[i]);
    dp[i][0] = max(dp[i][0], dp[i-1][1]);
    if(i % 2 == 0) {
      dp[i][1] = max(dp[i][1], dp[i-2][1] + a[i]);
    } else {
      dp[i][1] = max(dp[i][1], dp[i][0]);
    }
  }
  cout << dp[n-1][0] << endl;
  return 0;
}
