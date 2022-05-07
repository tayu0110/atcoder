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
  cin >> n;
  vector<pll> s(n), t(n);
  for(int i=0;i<n;i++) {
    ll a, b;
    cin >> a >> b;
    if(a < b) s.push_back({a, b});
    else t.push_back({b, a});
  }
  sort(s.begin(), s.end());
  sort(t.begin(), t.end(), greater<pll>());
  ll now = 0;
  ll x = 0;
  for(auto [a, b] : s) {
    now += a;
    x = max(now, x);
    now -= b;
  }
  for(auto [b, a] : t) {
    now += a;
    x = max(now, x);
    now -= b;
  }
  cout << x << endl;
  return 0;
}
