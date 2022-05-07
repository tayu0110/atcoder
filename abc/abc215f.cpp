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
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  sort(p.begin(), p.end());
  ll l = 0, r = 1001001001;
  while(r - l > 1) {
    ll m = (r + l) / 2;
    bool f = false;
    queue<pll> nt;
    ll mn = INF, mx = 0;
    for(auto [fi, se] : p) {
      while(!nt.empty()) {
        if(nt.front().first > fi - m) break;
        mn = min(mn, nt.front().second);
        mx = max(mx, nt.front().second);
        nt.pop();
      }
      if(mn <= se - m || mx >= se + m) {
        f = true;
        break;
      }
      nt.push({fi, se});
    }
    if(f) l = m;
    else r = m;
  }
  cout << l << endl;
  return 0;
}
