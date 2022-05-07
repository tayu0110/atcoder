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
tuple<int, int, int> rec(int now, int par, Graph &t) {
  queue<tuple<int, int, int>> nt;
  for(auto to : t[now]) {
    if(to == par) continue;
    nt.push(rec(to, now, t));
  }
  int size = 1;
  int first = 1, second = 0;
  vector<pii> even;
  vector<tuple<int, int, int>> odd;
  while(!nt.empty()) {
    auto [sz, fi, se] = nt.front();
    nt.pop();
    size += sz;
    if(sz % 2) {
      odd.push_back({fi - se, fi, se});
      continue;
    }
    if(se > fi) {
      second += se;
      first += fi;
      continue;
    }
    even.push_back({fi, se});
  }
  sort(odd.begin(), odd.end());
  for(int i=0;i<odd.size();i++) {
    auto [_, fi, se] = odd[i];
    if(i % 2) {
      first += se;
      second += fi;
    } else {
      first += fi;
      second += se;
    }
  }
  if(odd.size() % 2) {
    for(auto [fi, se] : even) {
      second += fi;
      first += se;
    }
  } else {
    for(auto [fi, se] : even) {
      second += se;
      first += fi;
    }
  }
  return make_tuple(size, first, second);
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> p(n);
  for(int i=1;i<n;i++) cin >> p[i], p[i]--;
  Graph t(n);
  for(int i=1;i<n;i++) {
    t[i].push_back(p[i]);
    t[p[i]].push_back(i);
  }
  auto [size, first, second] = rec(0, -1, t);
  DEBUG(size);DEBUG_EN(second);
  cout << first << endl;
  return 0;
}
