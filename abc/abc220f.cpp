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
vector<int> c;
vector<ll> d;
int child(int now, int par, Graph &t) {
  int res = 0;
  for(auto e : t[now]) {
    if(e == par) continue;
    res += child(e, now, t);
  }
  c[now] = res;
  return res+1;
}
void dfs(int now, int par, int dist, Graph &t) {
  d[now] = dist;
  for(auto e : t[now]) {
    if(e == par) continue;
    dfs(e, now, dist+1, t);
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
  c.assign(n, -1);
  d.assign(n, 0);
  child(0, -1, t);
  dfs(0, -1, 0, t);
  vector<ll> ans(n, -1);
  ans[0] = accumulate(d.begin(), d.end(), 0LL);
  queue<pii> nt;
  nt.push({0, -1});
  while(!nt.empty()) {
    auto p = nt.front();
    nt.pop();
    int now = p.first;
    int par = p.second;
    if(now != 0 && ans[now] >= 0) continue;
    if(now) ans[now] = ans[par] + n - 2 * c[now] - 2;
    for(auto e : t[now]) {
      if(ans[e] >= 0) continue;
      nt.push({e, now});
    }
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
