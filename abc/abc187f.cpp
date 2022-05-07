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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<int> t(n);
  for(int i=0;i<n;i++) t[i] = 1 << i;
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a] |= 1 << b;
    t[b] |= 1 << a;
  }
  vector<int> dp(1<<n, inf);
  for(int i=0;i<(1<<n);i++) {
    bool f = true;
    for(int j=0;j<n;j++) {
      if((i & (1<<j)) && (t[j] & i) != i) f = false;
    }
    if(f) dp[i] = 1;
  }
  for(int i=0;i<(1<<n);i++) for(int j=i;--j&=i;) {
    dp[i] = min(dp[i], dp[j] + dp[i ^ j]);
  }
  cout << dp[(1<<n)-1] << endl;
  return 0;
}
