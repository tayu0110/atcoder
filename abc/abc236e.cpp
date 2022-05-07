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
double solve1(int n, vector<ll> a) {
  ll l = 0, r = 1001001001001;
  vector<ll> b(n);
  while(r - l > 1) {
    ll m = (r + l) / 2;
    for(int i=0;i<n;i++) b[i] = a[i] - m;
    vector<vector<ll>> dp(2, vector<ll>(n+1, 0));
    for(int i=0;i<n;i++) {
      dp[0][i+1] = dp[1][i];
      dp[1][i+1] = max(dp[0][i], dp[1][i]) + b[i];
    }
    if(max(dp[0][n], dp[1][n]) < 0) r = m;
    else l = m;
  }
  return (double)l / 1000;
}
ll solve2(int n, vector<ll> a) {
  ll l = 0, r = 1001001001001;
  vector<int> b(n);
  while(r - l > 1) {
    ll m = (r + l) / 2;
    for(int i=0;i<n;i++) b[i] = (a[i] - m < 0 ? -1 : 1);
    vector<vector<int>> dp(2, vector<int>(n+1, 0));
    for(int i=0;i<n;i++) {
      dp[0][i+1] = dp[1][i];
      dp[1][i+1] = max(dp[0][i], dp[1][i]) + b[i];
    }
    if(max(dp[0][n], dp[1][n]) > 0) l = m;
    else r = m;
  }
  return l / 1000;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i], a[i] *= 1000;
  cout << solve1(n, a) << endl;
  cout << solve2(n, a) << endl;
  return 0;
}
