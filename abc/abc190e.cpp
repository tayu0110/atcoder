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
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  Graph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  int k;
  cin >> k;
  vector<int> c(k);
  map<int, int> mp;
  for(int i=0;i<k;i++) {
    cin >> c[i];
    c[i]--;
    mp[c[i]] = i;
  }
  vector<vector<ll>> d(k, vector<ll>(k));
  for(int i=0;i<k;i++) {
    vector<ll> nd(n, -1);
    nd[c[i]] = 0;
    queue<int> nt;
    nt.push(c[i]);
    while(!nt.empty()) {
      int now = nt.front();
      nt.pop();
      for(auto e : t[now]) {
        if(nd[e] >= 0) continue;
        nd[e] = nd[now] + 1;
        nt.push(e);
      }
    }
    for(int j=0;j<k;j++) d[mp[c[i]]][mp[c[j]]] = (nd[c[j]] >= 0 ? nd[c[j]] : INF);
  }
  vector<vector<ll>> dp(1<<k, vector<ll>(k, INF));
  for(int i=0;i<k;i++) dp[1<<i][i] = 0;
  for(int i=1;i<(1<<k);i++) {
    for(int j=0;j<k;j++) {
      if(!(i & (1<<j))) continue;
      for(int l=0;l<k;l++) {
        if(i & (1<<l)) continue;
        int nt = i | (1<<l);
        dp[nt][l] = min(dp[nt][l], dp[i][j] + d[j][l]);
      }
    }
  }
  ll ans = INF;
  for(int i=0;i<k;i++) ans = min(ans, dp[(1<<k)-1][i]);
  if(ans >= INF) cout << -1 << endl;
  else cout << ans + 1 << endl;
  return 0;
}
