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

using namespace std;

struct Edge {
  int to;
  long long c, d;
  Edge() : to(0), c(0), d(0) {}
  Edge(int to, long long c, long long d) : to(to), c(c), d(d) {}
  Edge(const Edge& e) {
    to = e.to;
    c = e.c;
    d = e.d;
  }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<pll, vector<pll>, greater<pll>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
ll wt(ll c, ll d, ll nt) {
  // cout << "c: " << c << " d: " << d << " nt: " << nt << endl;
  if(nt >= d) return 0;
  ll l = 0, r = d+1;
  auto f = [&](ll k) {
    return k + d / (nt+k+1);
  };
  set<pll> ck;
  while(r - l > 1) {
    if(ck.find({l, r}) != ck.end()) {
      ll fl = f(l);
      ll fr = f(r);
      if(f(r-1) < fl && f(r-1) < fr) r--;
      else if(f(l+1) < fr && f(l+1) < fl) l++;
      else break;
    }
    ck.insert({l, r});
    // cout << "l: " << l << " r: " << r << endl;
    ll c1 = (l*2+r) / 3;
    ll c2 = (l+r*2) / 3;
    ll s = f(c1);
    ll t = f(c2);
    if(s < t) r = c2;
    else l = c1;
  }
  // cout << "return: " << (f(l) < f(r) ? l : r) << endl;
  if(f(l) < f(r)) return l;
  else return r;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    ll a, b, c, d;
    cin >> a >> b >> c >> d;
    a--;b--;
    t[a].push_back(Edge(b, c, d));
    t[b].push_back(Edge(a, c, d));
  }
  heap nt;
  nt.push({0, 0});
  vector<ll> dist(n, INF);
  while(!nt.empty()) {
    auto tp = nt.top();
    nt.pop();
    int now = tp.second;
    ll nd = tp.first;
    if(dist[now] <= nd) continue;
    dist[now] = nd;
    for(int i=0;i<t[now].size();i++) {
      auto j = t[now][i];
      ll c = j.c;
      ll d = j.d;
      ll to = j.to;
      ll p = wt(c, d, nd);
      // cout << "p: " << p << endl;
      ll newd = nd + p + c + d / (nd+p+1);
      if(dist[to] > newd) nt.push({newd, to});
    }
  }
  if(dist[n-1] == INF) cout << -1 << endl;
  else cout << dist[n-1] << endl;
  return 0;
}
