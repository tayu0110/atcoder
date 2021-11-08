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
vector<ll> sum;
ll get_sum(int l, int r) {
  ll res = sum[r];
  if(l) res -= sum[l-1];
  return res;
}
vector<vector<ll>> dp;
ll rec(int l, int r) {
  if(dp[l][r] != INF) return dp[l][r];
  ll res = INF;
  for(int i=l;i<r;i++) res = min(res, rec(l, i) + rec(i+1, r) + get_sum(l, r));
  return dp[l][r] = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  sum.assign(n, 0);
  sum[0] = a[0];
  for(int i=1;i<n;i++) sum[i] = a[i] + sum[i-1];
  dp.assign(n, vector<ll>(n, INF));
  for(int i=0;i<n;i++) dp[i][i] = 0;
  cout << rec(0, n-1) << endl;
  return 0;
}
