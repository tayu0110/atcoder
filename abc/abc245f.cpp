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
set<int> ck;
set<int> res;
set<int> pending;
bool dfs(int now, int par, Graph &t) {
  bool f = false;
  pending.insert(now);
  for(auto to : t[now]) {
    if(res.find(to) != res.end() || pending.find(to) != pending.end()) {
      f = true;
      continue;
    }
    if(ck.find(to) != ck.end()) continue;
    f |= dfs(to, now, t);
  }
  pending.erase(now);
  ck.insert(now);
  if(f) {
    res.insert(now);
  }
  return f;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--; v--;
    t[u].push_back(v);
  }
  for(int i=0;i<n;i++) {
    if(ck.find(i) != ck.end()) continue;
    dfs(i, -1, t);
  }
  cout << res.size() << endl;
  return 0;
}
