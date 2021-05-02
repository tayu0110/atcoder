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
  long long weight;
  long long k;
  Edge(int to, long long weight, long long k) : to(to), weight(weight), k(k) {}
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, x, y;
  cin >> n >> m >> x >> y;
  weightedGraph g(n+1);
  for(int i=0;i<m;i++){
    ll a, b, t, k;
    cin >> a >> b >> t >> k;
    g[a].push_back(Edge(b, t, k));
    g[b].push_back(Edge(a, t, k));
  }
  vector<ll> d(n+1, INF);
  d[x] = 0;
  vector<bool> ck(n+1, false);
  priority_queue<pll, std::vector<pll>, std::greater<pll>> nt;
  nt.push(make_pair(0, x));
  while(!nt.empty()) {
    int now = nt.top().second;
    nt.pop();
    if(now == y) break;
    if(ck[now]) continue;
    ck[now] = true;
    // cout << "now: " << now << endl;
    for(int i=0;i<g[now].size();i++){
      Edge j = g[now][i];
      if(ck[j.to]) continue;
      ll t = d[now];
      if(t % j.k != 0) t = (t/j.k + 1) * j.k;
      ll dist = j.weight + t;
      if(d[j.to] > dist) {
        d[j.to] = dist;
        nt.push(make_pair(dist, j.to));
        // cout << "j.to: " << j.to << endl;
        // cout << "dist: " << dist << endl;
      }
    }
  }
  if(d[y] == INF) cout << -1 << endl;
  else cout << d[y];
  return 0;
}
