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
  for(int i=1;i<=t;i++) {
    int n, k;
    cin >> n >> k;
    vector<ll> e(n);
    for(int j=0;j<n;j++) cin >> e[j];
    if(n == 1) {
      printf("Case #%d: %d\n", i, 0);
      continue;
    }
    ll add = 0, mul = 0;
    for(int j=0;j<n;j++) add += e[j];
    for(int j=0;j<n;j++) {
      for(int l=j+1;l<n;l++) {
        mul += e[j] * e[l];
      }
    }
    // printf("Case%d: add: %d, mul: %d\n", i, add, mul);
    if(!add) {
      if(!mul) printf("Case #%d: 0\n", i);
      else printf("Case #%d: IMPOSSIBLE\n", i);
    } else {
      if(mul % add) printf("Case #%d: IMPOSSIBLE\n", i);
      else printf("Case #%d: %lld\n", i, mul / add * -1);
    }
  }
  return 0;
}
