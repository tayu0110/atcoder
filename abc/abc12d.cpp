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
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
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
  int n, m;
  cin >> n >> m;
  Graph dp(n+1, vector<int>(n+1, inf));
  for(int i=0;i<m;i++) {
    int a, b, t;
    cin >> a >> b >> t;
    dp[a][b] = t;
    dp[b][a] = t;
  }
  for(int i=0;i<n+1;i++) dp[i][i] = 0;
  for(int k=1;k<n+1;k++) for(int i=1;i<n+1;i++) for(int j=1;j<n+1;j++) dp[i][j] = min(dp[i][j], dp[i][k]+dp[k][j]);
  int ans = inf;
  for(int i=1;i<n+1;i++) {
    int mx = 0;
    for(int j=1;j<n+1;j++) {
      mx = max(mx, dp[i][j]);
    }
    ans = min(ans, mx);
  }
  cout << ans << endl;
  // Dijkstra
  // int n, m;
  // cin >> n >> m;
  // weightedGraph g(n+1, vector<Edge>(0));
  // for(int i=0;i<m;i++) {
  //   int a, b, t;
  //   cin >> a >> b >> t;
  //   g[a].push_back(Edge(b, t));
  //   g[b].push_back(Edge(a, t));
  // }
  // vector<vector<int>> dist(n+1, vector<int>(n+1, inf));
  // for(int i=1;i<n+1;i++) {
  //   priority_queue<pii, vector<pii>, greater<pii>> nt;
  //   nt.push(make_pair(0, i));
  //   while(!nt.empty()) {
  //     int now = nt.top().second;
  //     int d = nt.top().first;
  //     nt.pop();
  //     if(dist[i][now] != inf) continue;
  //     dist[i][now] = d;
  //     for(int j=0;j<g[now].size();j++) {
  //       int k = g[now][j].to;
  //       int kd = dist[i][now] + g[now][j].weight;
  //       if(dist[i][k] != inf) continue;
  //       nt.push(make_pair(kd, k));
  //     }
  //   }
  // }
  // int ans = inf;
  // for(int i=1;i<n+1;i++) {
  //   int mx = 0;
  //   for(int j=1;j<n+1;j++) {
  //     mx = max(mx, dist[i][j]);
  //   }
  //   ans = min(ans, mx);
  // }
  // cout << ans << endl;
  return 0;
}
