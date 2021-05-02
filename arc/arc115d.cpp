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
#define INF (1LL<<60)
#define inf (1<<29)
#define MAX_N 5005
#define MAX_M 5005

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<pii> ab(m+1);
  for(int i=1;i<m+1;i++) {
    int a, b;
    cin >> a >> b;
    ab[i] = make_pair(a, b);
  }
  ll dp[2][MAX_M][2][MAX_N];
  for(int i=0;i<n+1;i++) {
    dp[0][0][0][i] = 1;
    dp[0][0][1][i] = 0;
  }
  for(int i=1;i<m+1;i++) {
    int a = ab[i-1].first;
    int b = ab[i-1].second;
    for(int j=1;j<n+1;j++) {
      if(j == a) {
        dp[1][i][0][j] = dp[1][i-1][0][j] + dp[0][i-1][1][j];
        dp[1][i][1][j] = dp[1][i-1][1][j] + dp[0][i-1][0][j];
        dp[0][i][0][j] = dp[0][i-1][0][j] + dp[1][i-1][0][j];
        dp[0][i][1][j] = dp[0][i-1][1][j] + dp[1][i-1][1][j];
      }
    }
    dp[0][i][0][a] = dp[0][i-1][0][a];
    dp[0][i][1][a] = dp[0][i-1][1][a];
    dp[0][i][0][b] = dp[0][i-1][0][b];
    dp[0][i][1][b] = dp[0][i-1][1][b];
    dp[1][i][0][a] = dp[0][i-1][0][a] + 1;
    dp[1][i][1][a] = dp[0][i-1][1][a] + 1;
    dp[1][i][0][b] = dp[0][i-1][0][b] + 1;
    dp[1][i][1][b] = dp[0][i-1][1][b] + 1;
  }
  vector<ll> ck(n + 1);

  return 0;
}
