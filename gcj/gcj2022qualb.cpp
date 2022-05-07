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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  const int req = 1000000;
  for(int i=1;i<=t;i++) {
    int c, m, y, k;
    c = m = y = k = inf;
    for(int j=0;j<3;j++) {
      int nc, nm, ny, nk;
      cin >> nc >> nm >> ny >> nk;
      c = min(c, nc);
      m = min(m, nm);
      y = min(y, ny);
      k = min(k, nk);
    }
    if(c + m + y + k < req) {
      cout << "Case #" << i << ": IMPOSSIBLE" << endl;
      continue;
    }
    int rem = c + m + y + k - req;
    while(c && rem) c--, rem--;
    while(m && rem) m--, rem--;
    while(y && rem) y--, rem--;
    while(k && rem) k--, rem--;
    cout << "Case #" << i << ": " << c << " " << m << " " << y << " " << k << endl;
  }
  return 0;
}
