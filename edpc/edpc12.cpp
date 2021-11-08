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
int n;
const int m = 3010;
vector<vector<ll>> dp(m, vector<ll>(m, 0));
ll rec(int l, int r, int t, vector<ll> &a) {
  if(l == r) {
    if(t % 2) return dp[l][r] = -a[l];
    else return dp[l][r] = a[l];
  }
  if(dp[l][r] != INF && dp[l][r] != -INF) return dp[l][r];
  auto ch = t % 2 ? [](ll &p, ll q) {p = min(p, q);} : [](ll &p, ll q) {p = max(p, q);};
  auto f = t % 2 ? [](ll p) {return -p;} : [](ll p) {return p;};
  ll res = t % 2 ? INF : -INF;
  ch(res, rec(l+1, r, (t+1)%2, a) + f(a[l]));
  ch(res, rec(l, r-1, (t+1)%2, a) + f(a[r]));
  return dp[l][r] = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<m;i++) for(int j=0;j<m;j++) {
    if(i > j) continue;
    int t = i + (n-1) - j;
    if(t < 0) continue;
    if(t % 2) dp[i][j] = INF;
    else dp[i][j] = -INF;
  }
  rec(0, n-1, 0, a);
  cout << dp[0][n-1] << endl;
  return 0;
}
