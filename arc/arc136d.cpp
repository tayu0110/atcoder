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
bool check(int l, int r) {
  while(l && r) {
    int nl = l % 10;
    int nr = r % 10;
    if(nl + nr > 9) return false;
    l /= 10;
    r /= 10;
  }
  return true;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  const int mx = 1000000;
  vector<int> dp(mx);
  int t = 1;
  for(int i=0;i<n;i++) dp[a[i]]++;
  for(int i=0;i<6;i++) {
    for(int j=0;j<mx;j++) {
      if(j / t % 10 < 9) dp[j+t] += dp[j];
    }
    t *= 10;
  }
  ll ans = 0;
  for(int i=0;i<n;i++) {
    ans += dp[t - 1 - a[i]];
    if(check(a[i], a[i])) ans--;
  }
  cout << ans / 2 << endl;
  return 0;
}
