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
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  ll ans = 0;
  for(int i=0;i<n;i++) for(int j=i+1;j<n;j++) for(int k=j+1;k<n;k++) {
    vector<pll> q = {p[i], p[j], p[k]};
    sort(q.begin(), q.end());
    auto [ix, iy] = q[0];
    auto [jx, jy] = q[1];
    auto [kx, ky] = q[2];
    if(ix == jx && jx == kx) continue;
    if(iy == jy && jy == ky) continue;
    if((jy-iy)*(kx-jx) == (ky-jy)*(jx-ix)) continue;
    ans++;
  }
  cout << ans << endl;
  return 0;
}
