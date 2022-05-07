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

ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, p;
  string s;
  cin >> n >> p >> s;
  if(p == 2 || p == 5) {
    ll ans = 0;
    for(int i=0;i<n;i++) if((s[i] - '0') % p == 0) ans += i+1;
    cout << ans << endl;
    return 0;
  }
  ll t = 1;
  vector<ll> v(n+1);
  for(int i=n-1;i>=0;i--) {
    v[i] = t * (s[i] - '0') + v[i+1];
    v[i] %= p;
    t *= 10;
    t %= p;
  }
  map<ll, ll> mp;
  for(int i=0;i<n+1;i++) mp[v[i]]++;
  ll ans = 0;
  for(auto [f, s] : mp) {
    ans += s * (s - 1) / 2;
  }
  cout << ans << endl;
  return 0;
}
