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
int pop_count(ll d) {
  int res = 0;
  while(d) {
    res += (d & 1);
    d >>= 1;
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll x, y;
  cin >> n >> x >> y;
  vector<ll> a(n), b(n);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<n;i++) cin >> b[i];
  vector<ll> dp(1 << n, INF);
  dp[0] = 0;
  for(int i=0;i<(1<<n);i++) {
    int pc = pop_count(i);
    ll checked = 0;
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) continue;
      ll cost = y * checked + abs(a[j] - b[pc]) * x;
      dp[i | (1<<j)] = min(dp[i | (1<<j)], dp[i] + cost);
      checked++;
    }
  }
  cout << dp[(1<<n)-1] << endl;
  return 0;
}
