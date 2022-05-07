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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll l;
  cin >> n >> l;
  deque<tuple<ll, char, ll>> nt;
  for(int i=0;i<n;i++) {
    ll x;
    char d;
    cin >> x >> d;
    nt.push_back({x, d, 1});
  }
  ll ans = 0;
  ll start = 1, end = l;
  while(!nt.empty()) {
    auto [x, d, rn] = nt.front();
    if(d == 'R') break;
    nt.pop_front();
    ans += x - start;
    start++;
  }
  DEBUG_EN(ans);
  while(!nt.empty()) {
    auto [x, d, rn] = nt.back();
    if(d == 'L') break;
    nt.pop_back();
    ans += end - x;
    end--;
  }
  DEBUG_EN(ans);
  deque<tuple<ll, char, ll>> tmp;
  start = -1;
  while(!nt.empty()) {
    auto [x, d, rn] = nt.front();
    if(d == 'R') {
      tmp.push_back(nt.front());
      nt.pop_front();
      start = -1;
      continue;
    }
    if(start < 0) {
      tmp.push_back(nt.front());
      start = x+1;
    } else {
      auto &[_1, _2, frn] = tmp.back();
      frn++;
      ans += x - start;
      start++;
    }
    nt.pop_front();
  }
  DEBUG_EN(ans);
  nt.swap(tmp);
  start = -1;
  tmp.clear();
  while(!nt.empty()) {
    auto [x, d, rn] = nt.back();
    if(d == 'L') {
      tmp.push_front(nt.back());
      nt.pop_back();
      start = -1;
      continue;
    }
    if(start < 0) {
      tmp.push_front(nt.back());
      start = x-1;
    } else {
      auto &[_1, _2, brn] = tmp.front();
      brn++;
      ans += start - x;
      start--;
    }
    nt.pop_back();
  }
  DEBUG_EN(ans);
  while(!tmp.empty()) {
    auto [lx, ld, lrn] = tmp.front();
    tmp.pop_front();
    auto [rx, rd, rrn] = tmp.front();
    tmp.pop_front();
    if(lrn < rrn) swap(lrn, rrn);
    ans += lrn * (rx-lx-1);
  }
  DEBUG_EN(ans);
  cout << ans << endl;
  return 0;
}
