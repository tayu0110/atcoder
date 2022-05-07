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
ll ans = 0;
void solve(int now, vector<vector<ll>> &a, vector<int> &ck) {
  int m = ck.size();
  int f = -1;
  for(int i=0;i<m;i++) {
    if(ck[i] >= 0) continue;
    f = i;
    ck[i] = now;
    break;
  }
  if(f < 0) {
    ll res = 0;
    for(int i=0;i<m;i++) {
      for(int j=i+1;j<m;j++) {
        if(ck[i] == ck[j]) {
          res ^= a[i][j];
          break;
        }
      }
    }
    ans = max(res, ans);
    return;
  }
  for(int i=f+1;i<m;i++) {
    if(ck[i] >= 0) continue;
    ck[i] = now;
    solve(now+1, a, ck);
    ck[i] = -1;
  }
  ck[f] = -1;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  const int m = 2*n;
  vector<vector<ll>> a(2*n, vector<ll>(2*n));
  for(int i=0;i<2*n-1;i++) {
    for(int j=0;j<2*n;j++) {
      if(i >= j) continue;
      ll k;
      cin >> k;
      a[i][j] = k;
    }
  }
  vector<int> ck(m, -1);
  solve(0, a, ck);
  cout << ans << endl;
  return 0;
}
