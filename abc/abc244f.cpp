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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--; v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  vector<vector<int>> dp(1<<n, vector<int>(n, inf));
  for(int i=0;i<n;i++) dp[0][i] = 0;
  bool flag = true;
  while(flag) {
    flag = false;
    for(int i=0;i<(1<<n);i++) {
      for(int j=0;j<n;j++) {
        if(dp[i][j] == inf) continue;
        for(auto to : t[j]) {
          int st = i ^ (1<<to);
          if(dp[st][to] > dp[i][j]+1) {
            flag = true;
            dp[st][to] = dp[i][j]+1;
          }
        }
      }
    }
  }
  int ans = 0;
  for(int i=0;i<(1<<n);i++) {
    int mn = inf;
    for(int j=0;j<n;j++) mn = min(mn, dp[i][j]);
    ans += mn;
  }
  cout << ans << endl;
  return 0;
}
