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
ll k;
vector<vector<int>> dp;
int rec(int now, int f, vector<vector<ll>> &a) {
  if(dp[f][now] >= 0) return dp[f][now];
  if(now == a[0].size()-1) return 1;
  if(abs(a[0][now+1] - a[f][now]) <= k) {
    int res = rec(now+1, 0, a);
    if(res) {
      dp[f][now] = 1;
      return 1;
    }
  }
  if(abs(a[1][now+1] - a[f][now]) <= k) {
    int res = rec(now+1, 1, a);
    if(res) {
      dp[f][now] = 1;
      return 1;
    }
  }
  dp[f][now] = 0;
  return 0;
}

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n >> k;
  vector<vector<ll>> a(2, vector<ll>(n));
  for(int i=0;i<n;i++) cin >> a[0][i];
  for(int i=0;i<n;i++) cin >> a[1][i];
  dp.assign(2, vector<int>(n, -1));
  rec(0, 0, a);
  rec(0, 1, a);
  DEBUG(dp[0][0]);DEBUG_EN(dp[1][0]);
  if(dp[0][0] || dp[1][0]) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
