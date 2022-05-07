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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
ll sq(ll val) {
  ll m = sqrt(val)-1;
  while((m+1) <= val / (m+1)) m++;
  return m;
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  const int sq4 = 1000000;
  vector<ll> dp(sq4), sum(sq4);
  dp[1] = sum[1] = 1;
  for(int i=2;i<sq4;i++) {
    dp[i] = sum[sq(i)];
    sum[i] = sum[i-1] + dp[i];
  }
  while(t--) {
    ll x;
    cin >> x;
    ll y = sq(x);
    ll z = sq(y);
    ll res = 0;
    for(ll i=1;i<=z;i++) res += (y - i * i + 1) * dp[i];
    cout << res << endl;
  }
  return 0;
}
