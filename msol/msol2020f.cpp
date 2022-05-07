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
int solve(int n, vector<tuple<int, int, char>> &p) {
  const int mx = 200010;
  vector<vector<int>> u(mx), d(mx), l(mx), r(mx);
  for(int i=0;i<n;i++) {
    auto [x, y, c] = p[i];
    if(c == 'U') u[x].push_back(y);
    else if(c == 'D') d[x].push_back(y);
    else if(c == 'L') l[y].push_back(x);
    else r[y].push_back(x);
  }
  int res = inf;
  for(int i=0;i<mx;i++) {
    if(u[i].size()) sort(u[i].begin(), u[i].end());
    if(d[i].size()) sort(d[i].begin(), d[i].end());
    if(l[i].size()) sort(l[i].begin(), l[i].end());
    if(r[i].size()) sort(r[i].begin(), r[i].end());
    for(auto e : u[i]) {
      auto it = upper_bound(d[i].begin(), d[i].end(), e);
      if(it == d[i].end()) continue;
      res = min(res, (*it-e) * 5);
    }
    for(auto e : r[i]) {
      auto it = upper_bound(l[i].begin(), l[i].end(), e);
      if(it == l[i].end()) continue;
      res = min(res, (*it-e) * 5);
    }
  }
  return res;
}
int solve2(int n, vector<tuple<int, int, char>> &p) {
  vector<pii> u, d, l, r;
  for(int i=0;i<n;i++) {
    auto [x, y, c] = p[i];
    if(c == 'U') u.push_back({x, y});
    else if(c == 'D') d.push_back({x, y});
    else if(c == 'L') l.push_back({x, y});
    else r.push_back({x, y});
  }
  auto f = [](vector<pii> &u, vector<pii> &l, function<void(int&, int&)> uef, function<void(int &, int&)> lef) {
    map<int, set<int>> nu, nl;
    for(auto [x, y] : u) {
      uef(x, y);
      nu[y-x].insert(x);
    }
    for(auto [x, y] : l) {
      lef(x, y);
      nl[y-x].insert(x);
    }
    int res = inf;
    for(auto [f, s] : nu) {
      if(nl.find(f) == nl.end()) continue;
      set<int> &ck = nl[f];
      for(auto e : s) {
        auto it = ck.upper_bound(e);
        if(it == ck.end()) continue;
        res = min(res, (*it - e)*10);
      }
    }
    return res;
  };
  int res = inf;
  auto g = [](int &x, int &y) {};
  res = min(res, f(u, l, g, g));
  auto h = [](int &x, int &y) { swap(x, y); y = -y; };
  res = min(res, f(l, d, h, h));
  auto i = [](int &x, int &y) { x = -x; y = -y; };
  res = min(res, f(d, r, i, i));
  auto j = [](int &x, int &y) { swap(x, y); x = -x; };
  res = min(res, f(r, u, j, j));
  return res;
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<tuple<int, int, char>> p(n);
  for(int i=0;i<n;i++) {
    int x, y;
    char u;
    cin >> x >> y >> u;
    p[i] = {x, y, u};
  }
  int ans = inf;
  ans = min(ans, solve(n, p));
  ans = min(ans, solve2(n, p));
  if(ans == inf) cout << "SAFE" << endl;
  else cout << ans << endl;
  return 0;
}
