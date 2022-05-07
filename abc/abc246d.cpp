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
const ll INF = 1LL << 62;
const int inf = 1 << 29;
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  ll n;
  cin >> n;
  if(!n) {
    cout << 0 << endl;
    return 0;
  }
  const ll mx = 2000101;
  ll x = INF;
  auto f = [](ll a, ll b) { return (a+b) * (a*a+b*b); };
  DEBUG_EN(f(mx, 0));
  for(ll a=0;a<=mx;a++) {
    ll mxb = mx - a;
    ll l = 0, r = mxb+1;
    while(r-l > 1) {
      ll b = (r+l) / 2;
      ll k = f(a, b);
      if(k < n) l = b;
      else r = b;
    }
    ll k = f(a, r);
    if(k < n) continue;
    if(k > x) continue;
    x = min(x, k);
  }
  cout << x << endl;
  return 0;
}
