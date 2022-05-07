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
int comp(map<ll, int> &mp) {
  int cnt = 0;
  for(auto &[f, s] : mp) s = cnt++;
  return cnt;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll h, w, n;
  cin >> h >> w >> n;
  ll sx, sy, gx, gy;
  cin >> sx >> sy >> gx >> gy;
  vector<pll> p(n);
  map<ll, int> x_comp, y_comp;
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  map<ll, set<ll>> to_hor, to_ver;
  for(auto [x, y] : p) {
    to_hor[x].insert(y);
    to_ver[y].insert(x);
  }
  set<pll> ck;
  queue<tuple<ll, ll, int>> nt;
  nt.push({sx, sy, 0});
  while(!nt.empty()) {
    auto [x, y, d] = nt.front();
    nt.pop();
    if(ck.find({x, y}) != ck.end()) continue;
    ck.insert({x, y});
    if(x == gx && y == gy) {
      cout << d << endl;
      return 0;
    }
    if(to_hor.find(x) != to_hor.end()) {
      set<ll> &th = to_hor[x];
      auto it = th.lower_bound(y);
      if(it != th.end()) {
        ll ny = *it - 1;
        if(ck.find({x, ny}) == ck.end()) nt.push({x, ny, d+1});
      }
      if(it != th.begin()) {
        it--;
        ll ny = *it + 1;
        if(ck.find({x, ny}) == ck.end()) nt.push({x, ny, d+1});
      }
    }
    if(to_ver.find(y) != to_ver.end()) {
      set<ll> &tv = to_ver[y];
      auto it = tv.lower_bound(x);
      if(it != tv.end()) {
        ll nx = *it - 1;
        if(ck.find({nx, y}) == ck.end()) nt.push({nx, y, d+1});
      }
      if(it != tv.begin()) {
        it--;
        ll nx = *it + 1;
        if(ck.find({nx, y}) == ck.end()) nt.push({nx, y, d+1});
      }
    }
  }
  cout << -1 << endl;
  return 0;
}
