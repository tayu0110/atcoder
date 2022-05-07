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
  int n, m;
  cin >> n >> m;
  vector<int> a(n, -1);
  for(int i=0;i<m;i++) {
    int k, c;
    cin >> k >> c;
    k--;
    if(n > 1 && k == 0 && c == 0) {
      cout << -1 << endl;
      return 0;
    }
    if(a[k] < 0) a[k] = c;
    else if(a[k] == c) a[k] = c;
    else {
      cout << -1 << endl;
      return 0;
    }
  }
  if(n == 1 && a[0] < 0) a[0] = 0;
  if(n > 1 && a[0] < 0) a[0] = 1;
  int res = 0;
  for(int i=0;i<n;i++) {
    res *= 10;
    if(a[i] >= 0) res += a[i];
  }
  cout << res << endl;
  return 0;
}
