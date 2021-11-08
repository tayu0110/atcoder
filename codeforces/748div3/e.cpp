#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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
vector<int> d;
void dfs(int now, int par, int nd, Graph &t) {
  d[now] = nd;
  for(auto e : t[now]) {
    if(e == par) continue;
    if(d[e] >= 0) continue;
    dfs(e, now, nd+1, t);
  }
}
vector<int> level;
int solve(int now, int par, Graph &t) {
  if(t[now].size() == 1) {
    return level[now] = 1;
  }
  int mx = 0;
  for(auto e : t[now]) {
    if(e == par) continue;
    int r = solve(e, now, t);
    mx = max(mx, r);
  }
  return level[now] = mx+1;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<int> res;
  while(t--) {
    int n, k;
    cin >> n >> k;
    Graph t(n);
    for(int i=0;i<n-1;i++) {
      int a, b;
      cin >> a >> b;
      a--;b--;
      t[a].push_back(b);
      t[b].push_back(a);
    }
    if(n < 3) {
      res.push_back(0);
      continue;
    }
    d.assign(n, -1);
    dfs(0, -1, 0, t);
    int mx = *max_element(d.begin(), d.end());
    int pos = find(d.begin(), d.end(), mx) - d.begin();
    d.assign(n, -1);
    dfs(pos, -1, 0, t);
    mx = *max_element(d.begin(), d.end());
    auto it = find(d.begin(), d.end(), (mx+1) / 2);
    int center = it - d.begin();
    level.assign(n, -1);
    solve(center, -1, t);
    sort(level.begin(), level.end());
    if(level[n-2] != level[n-3]) level[n-1] = level[n-2];
    auto it2 = upper_bound(level.begin(), level.end(), k);
    int r = level.end() - it2;
    res.push_back(r);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
