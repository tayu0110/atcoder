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

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    ll l, r;
    cin >> l >> r;
    l--;
    ll mx = r;
    while(r-l > 1) {
      ll m = (l+r) / 2;
      ll i = m * 10;
      string s = "1" + to_string(m);
      ll j = stoll(s);
      if(min(i, j) > mx) r = m;
      else l = m;
    }
    cout << mx - l << endl;
  }
  return 0;
}
