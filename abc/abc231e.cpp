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
const ld PI = 3.141592653589793238462643383;
map<pll, ll> memo;
ll solve(ll rem, int used, vector<ll> &a) {
  if(rem == 0) return 0;
  if(used == a.size()-1) {
    if (rem % a[used] != 0) return -INF;
    else return rem / a[used];
  }
  ll r = rem % a[used+1];
  ll res1 = (a[used+1] - r) / a[used];
  ll t = rem + a[used+1] - r;
  if(memo.find({t, used+1}) != memo.end()) {
    res1 += memo[{t, used+1}];
  } else {
    memo[{t, used+1}] = solve(t, used+1, a);
    res1 += memo[{t, used+1}];
  }
  ll res2 = r / a[used];
  t = rem - r;
  if(memo.find({t, used+1}) != memo.end()) {
    res2 += memo[{t, used+1}];
  } else {
    memo[{t, used+1}] = solve(t, used+1, a);
    res2 += memo[{t, used+1}];
  }
  return min(res1, res2);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, x;
  cin >> n >> x;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  cout << solve(x, 0, a) << endl;
  return 0;
}
