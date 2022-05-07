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
  ll n, a, b;
  cin >> n >> a >> b;
  if(a + b > n + 1) {
    cout << -1 << endl;
    return 0;
  }
  if(a * b < n) {
    cout << -1 << endl;
    return 0;
  }
  ll idx = 0, cur = 1, l = n - a;
  vector<int> ans(n);
  for(int i=1;i<a+1;i++) {
    int tk = min(l, b - 1);
    l -= tk;
    tk++;
    cur += tk;
    for(int j=cur-1;j>=cur-tk;j--) {
      ans[idx++] = j;
    }
  }
  for(int i=0;i<n;i++) {
    if(i) cout << " "; cout << ans[i];
  }
  cout << endl;
  return 0;
}
