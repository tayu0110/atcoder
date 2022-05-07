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
  ll k;
  cin >> n >> k;
  map<ll, ll> mp;
  for(int i=0;i<n;i++) {
    ll a, b;
    cin >> a >> b;
    if(a > k) continue;
    mp[a] += b;
  }
  n = mp.size();
  vector<pll> p;
  for(auto [f, s] : mp) p.push_back({f, s});
  ll now = k;
  ll ans = 0;
  for(int i=0;i<=30;i++) {
    if(now <= k) {
      ll res = 0;
      for(int j=0;j<n;j++) {
        auto [f, s] = p[j];
        if((f | now) == now) res += s;
      }
      ans = max(ans, res);
    }
    now &= (0xFFFFFFFFLL << (i+1));
    for(int j=0;j<i;j++) now |= (1LL<<j);
  }
  cout << ans << endl;
  return 0;
}
