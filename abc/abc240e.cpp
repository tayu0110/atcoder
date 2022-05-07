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
vector<pii> p;
int cnt = 1;
void rec(int now, int par, Graph &t) {
  if(t[now].size() == 1 && now != 0) {
    p[now] = {cnt, cnt};
    cnt++;
    return;
  }
  auto &[mn, mx] = p[now];
  for(auto to : t[now]) {
    if(to == par) continue;
    rec(to, now, t);
    auto [nmn, nmx] = p[to];
    mn = min(mn, nmn);
    mx = max(mx, nmx);
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  p.assign(n, {inf, -inf});
  rec(0, -1, t);
  for(auto [l, r] : p) cout << l << " " << r << endl;
  return 0;
}
