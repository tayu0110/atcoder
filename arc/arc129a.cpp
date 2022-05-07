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
ll solve(ll r, ll n) {
  ll res = 0;
  for(int i=60;i>=0;i--) {
    ll m = 1LL << i;
    if((n & m) == 0) continue;
    if(m > r) continue;
    if(1LL << (i+1) <= r) res += 1LL<<i;
    else {
      ll tmp = (r & (m-1)) + 1;
      res += tmp;
    }
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n, l, r;
  cin >> n >> l >> r;
  ll r1 = solve(r, n);
  ll r2 = solve(l-1, n);
  cout << r1 - r2 << endl;
  return 0;
}
