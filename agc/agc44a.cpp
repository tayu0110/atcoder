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
ll n, a, b, c, d;
map<ll, ll> mp;
ll solve(ll n) {
  if(!n) return 0LL;
  if(n == 1) return d;
  if(mp.find(n) != mp.end()) return mp[n];
  ll res = INF;
  if(n < res / d) res = n * d;
  ll s = n/2 * 2;
  ll t = (n/2+1) * 2;
  res = min(res, abs(s-n)*d + a + solve(s/2));
  res = min(res, abs(t-n)*d + a + solve(t/2));
  s = n/3 * 3;
  t = (n/3+1) * 3;
  res = min(res, abs(s-n)*d + b + solve(s/3));
  res = min(res, abs(t-n)*d + b + solve(t/3));
  s = n/5 * 5;
  t = (n/5+1) * 5;
  res = min(res, abs(s-n)*d + c + solve(s/5));
  res = min(res, abs(t-n)*d + c + solve(t/5));
  return mp[n] = res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<ll> res;
  while(t--) {
    cin >> n >> a >> b >> c >> d;
    mp.clear();
    mp[0] = 0LL;
    mp[1] = d;
    mp[2] = min(d*2, d+a);
    res.push_back(solve(n));
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
