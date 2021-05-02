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
  pair<int, int> to;
  long long weight;
  Edge() : to({0, 0}), weight(0) {}
  Edge(pair<int, int> to, long long weight) : to(to), weight(weight) {}
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
using heap = priority_queue<pair<Edge, int>, vector<pair<Edge, int>>, greater<pair<Edge, int>>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m, k;
  cin >> n >> m >> k;
  if(k % 2 != 0) {
    for(int i=0;i<n;i++) {
      for(int j=0;j<m;j++) {
        cout << -1;
        if(j != m-1) cout << " ";
      }
      cout << endl;
    }
    return 0;
  }
  vector<vector<ll>> h(n, vector<ll>(m-1, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<m-1;j++) {
      cin >> h[i][j];
    }
  }
  vector<vector<ll>> v(n-1, vector<ll>(m, 0));
  for(int i=0;i<n-1;i++) {
    for(int j=0;j<m;j++) {
      cin >> v[i][j];
    }
  }
  // for(int i=0;i<n;i++) for(int j=0;j<m;j++) dp[0][i][j] = 0;
  vector<vector<ll>> ans(n, vector<ll>(m, 0));
  for(int p=0;p<n;p++) {
    for(int q=0;q<m;q++) {
      vector<vector<vector<ll>>> dp(k/2 + 1, vector<vector<ll>>(n, vector<ll>(m, INF)));
      dp[0][p][q] = 0;
      for(int l=0;l<k/2;l++) {
        for(int i=0;i<n;i++) {
          for(int j=0;j<m;j++) {
            if(dp[l][i][j] >= INF) continue;
            if(i-1 >= 0) dp[l+1][i-1][j] = min(dp[l+1][i-1][j], dp[l][i][j] + v[i-1][j]);
            if(i+1 < n) dp[l+1][i+1][j] = min(dp[l+1][i+1][j], dp[l][i][j] + v[i][j]);
            if(j-1 >= 0) dp[l+1][i][j-1] = min(dp[l+1][i][j-1], dp[l][i][j] + h[i][j-1]);
            if(j+1 < m) dp[l+1][i][j+1] = min(dp[l+1][i][j+1], dp[l][i][j] + h[i][j]);
          }
        }
      }
      ll mn = INF;
      for(int i=0;i<n;i++) for(int j=0;j<m;j++) mn = min(mn, dp[k/2][i][j]);
      ans[p][q] = mn * 2;
    }
  }
  for(int i=0;i<n;i++) {
    for(int j=0;j<m;j++) {
      cout << ans[i][j];
      if(j != m-1) cout << " ";
    }
    cout << endl;
  }
  return 0;
}
