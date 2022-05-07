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
ll x, y;
map<ll, ll> memo;
ll solve(ll now) {
  if(x == now) return 0;
  if(memo.find(now) != memo.end()) return memo[now];
  ll res = abs(x - now);
  if(now < 2) return memo[now] = res;
  if(now % 2 == 0) {
    res = min(res, solve(now / 2) + 1);
    // DEBUG(now);DEBUG_EN(res);
    return memo[now] = res;
  }
  if(memo.find(now) == memo.end()) res = min(res, solve(now-1) + 1);
  if(memo.find(now) == memo.end()) res = min(res, solve(now+1) + 1);
  // DEBUG(now);DEBUG_EN(res);
  return memo[now] = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> x >> y;
  cout << solve(y) << endl;
  return 0;
}
