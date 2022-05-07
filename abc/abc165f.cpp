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
vector<ll> ans;
vector<ll> a;
void dfs(int now, int par, Graph &t, vector<ll> &lis) {
  int pos = lower_bound(lis.begin(), lis.end(), a[now]) - lis.begin();
  int tmp = -1;
  if(pos == lis.size()) {
    lis.push_back(a[now]);
  } else {
    tmp = lis[pos];
    lis[pos] = a[now];
  }
  ans[now] = lis.size();
  for(auto e : t[now]) {
    if(e == par) continue;
    dfs(e, now, t, lis);
  }
  if(tmp < 0) lis.pop_back();
  else lis[pos] = tmp;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  a.assign(n, 0);
  for(int i=0;i<n;i++) cin >> a[i];
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int v, u;
    cin >> v >> u;
    v--;u--;
    t[v].push_back(u);
    t[u].push_back(v);
  }
  ans.assign(n, -1);
  vector<ll> lis;
  dfs(0, -1, t, lis);
  for(auto e : ans) cout << e << endl;
  return 0;
}
