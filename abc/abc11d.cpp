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
  ll d, x, y;
  cin >> n >> d >> x >> y;
  x = abs(x);
  y = abs(y);
  if(x % d != 0 || y % d != 0) {
    cout << 0 << endl;
    return 0;
  }
  x /= d;
  y /= d;
  if(x + y > n) {
    cout << 0 << endl;
    return 0;
  }
  if((n-x-y) % 2 != 0) {
    cout << 0 << endl;
    return 0;
  }
  int diff = (n - x - y) / 2;
  double ans = 0;
  for(int k=0;k<diff+1;k++) {
    double tmp = 1;
    int cnt = 1;
    int up = y + k;
    int down = k;
    int right = x + diff - k;
    int left = diff - k;
    for(int i=1;i<=up;i++) {
      tmp *= cnt++;
      tmp /= i * 4;
    }
    for(int i=1;i<=down;i++) {
      tmp *= cnt++;
      tmp /= i * 4;
    }
    for(int i=1;i<=right;i++) {
      tmp *= cnt++;
      tmp /= i * 4;
    }
    for(int i=1;i<=left;i++) {
      tmp *= cnt++;
      tmp /= i * 4;
    }
    ans += tmp;
  }
  cout << ans << endl;
  return 0;
}
