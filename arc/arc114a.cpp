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
  int n;
  cin >> n;
  vector<int> x(n);
  for(int i=0;i<n;i++) cin >> x[i];
  vector<ll> p = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47};
  ll ans = 1;
  for(auto e : p) ans *= e;
  int m = p.size();
  for(int i=1;i<(1<<m);i++) {
    ll tmp = 1;
    for(int j=0;j<m;j++) {
      if(i & (1<<j)) tmp *= p[j];
    }
    bool f = true;
    for(int j=0;j<n;j++) {
      if(gcd(x[j], tmp) == 1) {
        f = false;
        break;
      }
    }
    if(f) ans = min(ans, tmp);
  }
  cout << ans << endl;
  return 0;
}
