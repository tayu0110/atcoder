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

using namespace std;

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
ll dfs(int now, ll &ans, vector<ll> &f, Graph &t) {
  if(!t[now].size()) return f[now];
  ll mn = INF;
  for(auto to : t[now]) {
    ll res = dfs(to, ans, f, t);
    if(res < mn) {
      if(mn != INF) ans += mn;
      mn = res;
    } else {
      ans += res;
    }
  }
  return max(f[now], mn);
}
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int _=1;_<=t;_++) {
    int n;
    cin >> n;
    vector<ll> f(n);
    vector<int> p(n);
    for(int i=0;i<n;i++) cin >> f[i];
    for(int i=0;i<n;i++) cin >> p[i], p[i]--;
    Graph g(n);
    set<int> rt;
    for(int i=0;i<n;i++) {
      if(p[i] < 0) {
        rt.insert(i);
        continue;
      }
      g[p[i]].push_back(i);
    }
    ll ans = 0;
    for(auto root : rt) {
      ans += dfs(root, ans, f, g);
    }
    printf("Case #%d: %lld\n", _, ans);
  }
  return 0;
}
