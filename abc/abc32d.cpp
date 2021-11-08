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

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
void f(vector<pll> &p, vector<pll> &res) {
  int n = p.size();
  for(int i=0;i<(1<<n);i++) {
    ll w = 0, v = 0;
    for(int j=0;j<n;j++) {
      if(i & (1 << j)) {
        w += p[j].second, v += p[j].first;
      }
    }
    res.push_back({w, v});
  }
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  ll w;
  cin >> n >> w;
  vector<pll> p(n);
  ll mxv = 0, mxw = 0;
  for(int i=0;i<n;i++) {
    ll v, w;
    cin >> v >> w;
    p[i] = {v, w};
    mxv = max(mxv, v);
    mxw = max(mxw, w);
  }
  if(n <= 30) {
    ll a = n/2, b = n-a;
    vector<pll> s(a), t(b);
    for(int i=0;i<a;i++) s[i] = p[i];
    for(int i=a;i<n;i++) t[i-a] = p[i];
    vector<pll> l, r;
    f(s, l); f(t, r);
    ll ans = 0;
    sort(l.begin(), l.end());
    vector<ll> mx(l.size());
    mx[0] = l[0].second;
    for(int i=1;i<l.size();i++) mx[i] = max(mx[i-1], l[i].second);
    for(int i=0;i<r.size();i++) {
      ll nw = r[i].first, nv = r[i].second;
      if(nw > w) continue;
      auto it = upper_bound(l.begin(), l.end(), make_pair(w-nw, INF));
      if(it == l.begin()) {
        ans = max(ans, nv);
        continue;
      }
      it--;
      int pos = it-l.begin();
      ans = max(ans, nv + mx[pos]);
    }
    for(int i=0;i<l.size();i++) if(l[i].first <= w) ans = max(ans, l[i].second);
    for(int i=0;i<r.size();i++) if(r[i].first <= w) ans = max(ans, r[i].second);
    cout << ans << endl;
  } else if(mxv <= 1000) {
    vector<ll> dp(n*mxv+1, INF);
    dp[0] = 0;
    for(int i=0;i<n;i++) for(int j=n*mxv;j>=0;j--) {
      if(dp[j] == INF) continue;
      if(j + p[i].first >= dp.size()) continue;
      if(dp[j] + p[i].second > w) continue;
      dp[j+p[i].first] = min(dp[j+p[i].first], dp[j] + p[i].second);
    }
    for(int i=dp.size()-1;i>=0;i--) {
      if(dp[i] != INF) {
        cout << i << endl; 
        break;
      }
    }
  } else {
    vector<ll> dp(n*mxw+1, -1);
    dp[0] = 0;
    for(int i=0;i<n;i++) for(int j=n*mxw;j>=0;j--) {
      if(dp[j] < 0) continue;
      ll nv = p[i].first, nw = p[i].second;
      if(j + nw >= min(w+1, (ll)dp.size())) continue;
      dp[j+nw] = max(dp[j+nw], dp[j] + nv);
    }
    ll ans = 0;
    for(int i=0;i<dp.size();i++) if(dp[i] >= 0) ans = max(ans, dp[i]);
    cout << ans << endl;
  }
  return 0;
}
