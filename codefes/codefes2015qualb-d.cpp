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
  int n;
  scanf("%d", &n);
  vector<ll> res;
  set<pll> ck;
  ck.insert({-1, -2});
  ck.insert({INF*2, INF});
  for(int i=0;i<n;i++) {
    ll s, c;
    scanf("%lld %lld", &s, &c);
    auto [r, l] = *ck.lower_bound({s, -1});
    if(l <= s) {
      c += r-l+1;
      s = l;
      ck.erase({r, l});
    } else l = s;
    while(c) {
      auto it = ck.lower_bound({s, -1});
      auto [nr, nl] = *it;
      if(nl == s+c) {
        ck.erase({nr, nl});
        ck.insert({nr, l});
        res.push_back(s+c-1);
        break;
      }
      if(nl > s+c) {
        ck.insert({s+c-1, l});
        res.push_back(s+c-1);
        break;
      }
      ck.erase({nr, nl});
      c -= nl - s;
      s = nr+1;
    }
  }
  for(auto e : res) printf("%lld\n", e);

  return 0;
}
