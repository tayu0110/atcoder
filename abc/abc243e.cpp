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
  vector<tuple<int, int, ll>> p(m);
  for(int i=0;i<m;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--; b--;
    p[i] = {a, b, c};
  }
  vector<vector<ll>> dp(n, vector<ll>(n, INF));
  for(int i=0;i<m;i++) {
    auto [a, b, c] = p[i];
    dp[a][b] = c;
    dp[b][a] = c;
  }
  for(int k=0;k<n;k++) for(int i=0;i<n;i++) for(int j=0;j<n;j++) dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
  int ans = 0;
  for(int i=0;i<m;i++) {
    auto [a, b, c] = p[i];
    if(dp[a][b] < c) {
      ans++;
      continue;
    }
    for(int k=0;k<n;k++) {
      if(k == a || k == b) continue;
      if(dp[a][k] + dp[k][b] == c) {
        ans++;
        break;
      }
    }
  }
  cout << ans << endl;
  return 0;
}
