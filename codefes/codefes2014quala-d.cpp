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
int popcount(int k) { return __builtin_popcount(k); }
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll a, k;
  cin >> a >> k;
  ll ten = 1;
  while(ten * 10 <= a) ten *= 10;
  vector<vector<ll>> dp(1<<10, vector<ll>(3, INF));
  for(int i=0;i<(1<<10);i++) dp[i][0] = -INF;
  dp[0][0] = 0;
  while(ten) {
    vector<vector<ll>> tmp(1<<10, vector<ll>(3, INF));
    for(int i=0;i<(1<<10);i++) tmp[i][0] = -INF;
    for(int i=0;i<(1<<10);i++) {
      for(int j=0;j<10;j++) {
        int ni = i | (1<<j);
        if(ni == 1) ni = 0;
        ll m = ten * j;
        for(int is=0;is<3;is++) {
          ll t = dp[i][is] + m;
          if(t < a) tmp[ni][0] = max(tmp[ni][0], t);
          else if(t == a) tmp[ni][1] = t;
          else tmp[ni][2] = min(tmp[ni][2], t);
        }
      }
    }
    ten /= 10;
    dp.swap(tmp);
  }
  ll ans = INF;
  for(int i=0;i<(1<<10);i++) {
    if(popcount(i) > k) continue;
    if(dp[i][0] < a) ans = min(ans, abs(a - dp[i][0]));
    if(dp[i][1] == a) ans = 0;
    if(dp[i][2] > a) ans = min(ans, abs(a - dp[i][2]));
  }
  cout << ans << endl;
  return 0;
}
