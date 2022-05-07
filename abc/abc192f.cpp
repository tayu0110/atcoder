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
  ll x;
  cin >> n >> x;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<vector<ll>> dp;
  ll ans = INF;
  for(int l=1;l<n+1;l++) {
    dp.assign(l+1, vector<ll>(l, -1));
    dp[0][0] = 0;
    for(int i=0;i<n;i++) {
      ll t = a[i];
      for(int j=l-1;j>=0;j--) {
        for(int k=0;k<l;k++) {
          if(dp[j][k] < 0) continue;
          int r = (dp[j][k] + t) % l;
          dp[j+1][r] = max(dp[j+1][r], dp[j][k] + t);
        }
      }
    }
    if(dp[l][x % l] < 0) continue;
    ll res = max(0LL, x - dp[l][x % l]) / l;
    ans = min(ans, res);
  }
  cout << ans << endl;
  return 0;
}
