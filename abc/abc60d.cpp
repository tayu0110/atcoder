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
  int n, w;
  cin >> n >> w;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    int u, v;
    cin >> u >> v;
    p[i] = {u, v};
  }
  sort(p.begin(), p.end(), greater<pll>());
  map<ll, ll> dp;
  dp[0] = 0;
  for(int i=0;i<n;i++) {
    auto [u, v] = p[i];
    map<ll, ll> tmp;
    for(auto [f, s] : dp) {
      tmp[f] = max(tmp[f], s);
      if(f + u > w) continue;
      tmp[f+u] = max(tmp[f+u], s+v);
    }
    tmp.swap(dp);
  }
  ll ans = 0;
  for(auto [f, s] : dp) ans = max(ans, s);
  cout << ans << endl;
  return 0;
}
