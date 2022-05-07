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
  int a, b, x;
  cin >> a >> b >> x;
  double res = 0;
  if(a * a * b > 2 * x) {
    DEBUG_EN("reached1");
    double l = 0, r = a;
    while(r-l > 0.0000001) {
      double m = (r+l) / 2;
      if(m * b * a <= 2 * x) l = m;
      else r = m;
    }
    res = atan2(b, r);
  } else if(a * a * b == 2 * x) {
    DEBUG_EN("reached2");
    res = atan2(b, a);
  } else {
    DEBUG_EN("reached3");
    double l = 0, r = b;
    while(r-l > 0.0000001) {
      double m = (r+l) / 2;
      if(m * a * a + ((double)b-m) * a * a * 2 <= 2 * x) r = m;
      else l = m;
    }
    res = atan2(r, a);
  }
  cout << res * 180 / acos(-1) << endl;
  return 0;
}
