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
  cin >> n;
  vector<vector<ll>> a(n, vector<ll>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) cin >> a[i][j];
  const int sz = 1<<n;
  vector<ll> dp(sz);
  for(int i=0;i<sz;i++) {
    vector<int> t;
    ll p = 0;
    for(int j=0;j<n;j++) if(i & (1<<j)) t.push_back(j);
    for(int j=0;j<t.size();j++) for(int k=j+1;k<t.size();k++) p += a[t[j]][t[k]];
    dp[i] = p;
  }
  for(int i=0;i<sz;i++) {
    for(int j=0;j<i;j++) {
      if(i & j) continue;
      dp[i | j] = max(dp[i | j], dp[i] + dp[j]);
    }
  }
  cout << dp[sz-1] << endl;
  return 0;
}
