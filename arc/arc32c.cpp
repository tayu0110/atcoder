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
  vector<tuple<int, int, int>> p(n);
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    p[i] = {a, b, i};
  }
  // sort(p.begin(), p.end(), [](auto l, auto r) {
  //   auto [_l, lb, __l] = l;
  //   auto [_r, rb, __r] = r;
  //   return lb < rb;
  // });
  sort(p.begin(), p.end());
  const int mx = 1000010;
  vector<pii> memo(mx, make_pair(0, -inf));
  int idx = n-1;
  for(int i=mx-2;i>=0;i--) {
    memo[i] = max(memo[i], memo[i+1]);
    while(idx >= 0) {
      auto [na, nb, ni] = p[idx];
      if(na < i) break;
      idx--;
      auto [s, t] = memo[nb];
      pii tmp = make_pair(s+1, -ni);
      memo[i] = max(memo[i], tmp);
    }
  }
  // for(int i=0;i<16;i++) {
  //   auto [a, idx] = memo[i];
  //   DEBUG(i);DEBUG(a);DEBUG_EN(idx);
  // }
  sort(p.begin(), p.end(), [](auto l, auto r) {
    auto [_l, __l, li] = l;
    auto [_r, __r, ri] = r;
    return li < ri;
  });
  idx = 0;
  vector<int> res;
  while(idx < mx) {
    auto [k, i] = memo[idx];
    if(-i == inf) break;
    res.push_back(-i+1);
    // DEBUG(-i);DEBUG_EN(idx);
    auto [a, b, _] = p[-i];
    idx = b;
  }
  cout << res.size() << endl;
  for(int i=0;i<res.size();i++) {
    if(i) cout << " "; cout << res[i];
  }
  cout << endl;
  return 0;
}
