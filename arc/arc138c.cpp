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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

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
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    ll a;
    cin >> a;
    p[i] = {a, i};
  }
  sort(p.begin(), p.end(), greater<pll>());
  ll ans = 0;
  for(int i=0;i<n/2;i++) {
    ans += p[i].first;
    p[i].first = -1;
  }
  for(int i=n/2;i<n;i++) p[i].first = 1;
  sort(p.begin(), p.end(), [](pll l, pll r) {
    return l.second < r.second;
  });
  int mn = inf;
  int now = 0;
  int pos = -1;
  for(int i=0;i<n;i++) {
    auto [l, r] = p[i];
    now += l;
    if(now < mn) {
      mn = now;
      pos = (i+1) % n;
    }
  }
  cout << pos << " " << ans << endl;
  return 0;
}
