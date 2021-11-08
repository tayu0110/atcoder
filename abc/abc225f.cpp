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
int pop_count(ll mask) {
  int res = 0;
  while(mask) {
    res += (mask & 1);
    mask >>= 1;
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<string> s(n);
  int len = 0;
  for(int i=0;i<n;i++) cin >> s[i], len += s[i].length();
  vector<vector<pair<string, ll>>> dp(k+1, vector<pair<string, ll>>(len+1));
  for(int i=0;i<k;i++) {
    for(int j=0;j<n;j++) {
      for(int l=len;l>=0;l--) {
        if(dp[i][l].first.length() != l) continue;
        if(pop_count(dp[i][l].second) != i) continue;
        ll used = dp[i][l].second;
        if(used & (1LL<<j)) continue;
        string tmp = dp[i][l].first + s[j];
        int nl = tmp.length();
        if(dp[i+1][nl].first.length() != nl) dp[i+1][nl] = {tmp, used | (1LL<<j)};
        else if(dp[i+1][nl].first > tmp) {
          dp[i+1][nl] = {tmp, used | (1LL<<j)};
        }
      }
    }
  }
  string ans;
  for(int i=0;i<len+10;i++) ans += "z";
  for(int i=1;i<len+1;i++) {
    if(dp[k][i].first.length() != i) continue;
    ans = min(ans, dp[k][i].first);
  }
  cout << ans << endl;
  return 0;
}
