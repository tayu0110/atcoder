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
vector<vector<int>> db;
vector<int> dist;
void dfs(int now, int par, int d, Graph &t) {
  dist[now] = d;
  db[now][0] = par;
  for(auto e : t[now]) {
    if(e == par) continue;
    dfs(e, now, d+1, t);
  }
}
void doubling() {
  int n = db.size();
  int k = db[0].size();
  for(int i=1;i<k;i++) {
    bool f = false;
    for(int j=0;j<n;j++) {
      if(db[j][i-1] < 0) {
        db[j][i] = -1;
        continue;
      }
      db[j][i] = db[db[j][i-1]][i-1];
      if(db[j][i] >= 0) f = true;
    }
    if(!f) break;
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
    int x, y;
    cin >> x >> y;
    x--;y--;
    t[x].push_back(y);
    t[y].push_back(x);
  }
  int mxk = 0;
  while((1<<mxk) <= n) mxk++;
  db.assign(n, vector<int>(mxk+1, -1));
  dist.assign(n, -1);
  dfs(0, -1, 0, t);
  doubling();
  int q;
  cin >> q;
  while(q--) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    if(!a || !b) {
      int mx = max(a, b);
      cout << dist[mx] + 1 << endl;
      continue;
    }
    int na = a, nb = b;
    int l = dist[na], r = dist[nb];
    if(l < r) {
      swap(na, nb);
      swap(l, r);
    }
    int diff = l - r;
    for(int i=mxk;i>=0;i--) if((1<<i) & diff) na = db[na][i];
    if(na == nb) {
      cout << dist[a] + dist[b] - 2 * dist[na] + 1 << endl;
      continue;
    }
    for(int i=mxk;i>=0;i--) {
      if(db[na][i] == db[nb][i]) continue;
      na = db[na][i];
      nb = db[nb][i];
    }
    int lca = db[na][0];
    cout << dist[a] + dist[b] - 2 * dist[lca] + 1 << endl;
  }
  return 0;
}
