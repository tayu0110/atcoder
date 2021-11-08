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
using Graph = vector<set<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<ll> a;
int cnt;
ll dfs(int now, int par, int tar, Graph &t) {
  ll res = a[now];
  for(auto e : t[now]) {
    if(e == par) continue;
    ll r = dfs(e, now, tar, t);
    if(cnt == 2) return -1;
    if(r == tar) {
      cnt++;
      if(cnt == 2) return -1;
    } else {
      res ^= r;
    }
  }
  return res;
}
bool solve(int tar, Graph &t) {
  cnt = 0;
  dfs(0, -1, tar, t);
  if(cnt == 2) return true;
  else return false;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<string> res;
  while(t--) {
    int n, k;
    cin >> n >> k;
    a.assign(n, 0);
    for(int i=0;i<n;i++) cin >> a[i];
    Graph t(n);
    for(int i=0;i<n-1;i++) {
      int u, v;
      cin >> u >> v;
      u--;v--;
      t[u].insert(v);
      t[v].insert(u);
    }
    int now = 0;
    for(int i=0;i<n;i++) now ^= a[i];
    if(now == 0) {
      res.push_back("YES");
      continue;
    }
    if(k == 2) {
      res.push_back("NO");
      continue;
    }
    if(solve(now, t)) {
      res.push_back("YES");
    } else {
      res.push_back("NO");
    }
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
