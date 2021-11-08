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
ll vtol(vector<int> &p) {
  ll res = 0;
  for(auto e : p) res = res*10 + (ll)e;
  return res;
}
vector<int> ltov(ll l) {
  vector<int> res;
  while(l) {
    res.push_back(l%10);
    l /= 10;
  }
  reverse(res.begin(), res.end());
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  const int n = 9;
  int m;
  cin >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  vector<int> p(n-1, 0);
  for(int i=0;i<n-1;i++) cin >> p[i];
  vector<int> q(n, 9);
  for(int i=0;i<n-1;i++) {
    q[p[i]-1] = i+1;
  }
  ll v = vtol(q);
  map<ll, set<ll>> mp;
  p.assign(n, 1);
  for(int i=1;i<n;i++) p[i] = p[i-1]+1;
  sort(p.begin(), p.end());
  do {
    ll v = vtol(p);
    int idx = -1;
    for(int i=0;i<n;i++) if(p[i] == 9) idx = i;
    for(auto e : t[idx]) {
      swap(p[e], p[idx]);
      ll nt = vtol(p);
      mp[v].insert(nt);
      swap(p[e], p[idx]);
    }
  } while(next_permutation(p.begin(), p.end()));
  queue<pll> nt;
  nt.push({v, 0});
  map<ll, ll> dist;
  while(!nt.empty()) {
    auto [now, nd] = nt.front();
    nt.pop();
    if(dist.find(now) != dist.end()) continue;
    dist[now] = nd;
    for(auto e : mp[now]) {
      if(dist.find(e) != dist.end()) continue;
      nt.push({e, nd+1});
    }
  }
  if(dist.find(123456789) == dist.end()) cout << -1 << endl;
  else cout << dist[123456789] << endl;
  return 0;
}
