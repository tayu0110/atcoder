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

using namespace std;

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
  cout << fixed << setprecision(20);
  int n, m, y, z;
  cin >> n >> m >> y >> z;
  vector<int> p(m);
  map<char, int> mp;
  for(int i=0;i<m;i++) {
    char c;
    int d;
    cin >> c >> d;
    mp[c] = i;
    p[i] = d;
  }
  string s;
  cin >> s;
  vector<vector<int>> dp(1<<m, vector<int>(m+1, -inf));
  dp[0][m] = 0;
  for(int i=0;i<n;i++) {
    char c = s[i];
    int nc = mp[c];
    vector<vector<int>> tmp(1<<m, vector<int>(m+1, -inf));
    for(int j=0;j<(1<<m);j++) {
      for(int k=0;k<m+1;k++) {
        if(dp[j][k] == -inf) continue;
        int nj = j | (1 << nc);
        int nt = dp[j][k] + p[nc];
        if(k == nc) nt += y;
        tmp[nj][nc] = max(tmp[nj][nc], nt);
        tmp[j][k] = max(tmp[j][k], dp[j][k]);
      }
    }
    tmp.swap(dp);
  }
  for(int i=0;i<m+1;i++) if(dp[(1<<m)-1][i] != inf) dp[(1<<m)-1][i] += z;
  int ans = -inf;
  for(int i=0;i<(1<<m);i++) for(int j=0;j<m+1;j++) ans = max(ans, dp[i][j]);
  cout << ans << endl;
  return 0;
}
