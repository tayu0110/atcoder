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

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<ll> res;
  while(t--) {
    ll n, a, b, x, y, z;
    cin >> n >> a >> b >> x >> y >> z;
    y = min(y, a * x);
    z = min(z, b * x);
    if(y * b > z * a) {
      swap(y, z);
      swap(a, b);
    }
    ll ans = x * n;
    if(n/a < a-1) for(ll i=0;i<=n;i+=a) ans = min(ans, i/a*y + (n-i)/b*z + (n-i)%b*x);
    else for(ll i=0;i<a;i++) {
      if(i*b > n) continue;
      ans = min(ans, (n-i*b)/a*y + i*z + (n-i*b)%a*x);
    }
    res.push_back(ans);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
