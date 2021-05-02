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
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> x(n), y(n), z(n);
  for(int i=0;i<n;i++) {
    cin >> x[i] >> y[i] >> z[i];
  }
  vector<vector<ll>> d(n, vector<ll>(n));
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      ll dist = abs(x[j] - x[i]) + abs(y[j] - y[i]) + max(0, z[j] - z[i]);
      d[i][j] = dist;
    }
  }
  vector<vector<ll>> dp(1<<n, vector<ll>(n, INF));
  dp[1][0] = 0;
  for(int i = 1; i < (1<<n); i++) {
    if(!(i & 1)) continue;
    for(int j = 0; j < n; j++) {
      if(!(i & (1<<j))) continue;
      for(int k = 0; k < n; k++) {
        int l = (i | (1<<k));
        dp[l][k] = min(dp[l][k], dp[i][j] + d[j][k]);
      }
    }
  }
  int t = (1<<n)-1;
  cout << dp[t][0] << endl;
  return 0;
}
