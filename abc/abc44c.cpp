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
  Edge(int to, long long weight) : to(to), weight(weight) {}
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
  int n, a;
  cin >> n >> a;
  vector<int> x(n+1);
  for(int i=1;i<n+1;i++) {
    cin >> x[i];
  }
  vector<vector<vector<ll>>> dp(n+1, vector<vector<ll>>(n+1, vector<ll>(n*a+1, 0)));
  for(int i=0;i<n*a+1;i++) {
    for(int k=0;k<n+1;k++) {
      for(int j=0;j<n+1;j++) {
        if(j==0 && k==0 && i==0) dp[j][k][i] = 1;
        else if(j>=1 && i-x[j]<0) dp[j][k][i] = dp[j-1][k][i];
        else if(j>=1 && i-x[j]>=0 && k>=1) dp[j][k][i] = dp[j-1][k][i] + dp[j-1][k-1][i-x[j]];
        else dp[j][k][i] = 0;
      }
    }
  }
  ll ans = 0;
  for(int k=1;k<n+1;k++) {
    ans += dp[n][k][k*a];
  }
  cout << ans << endl;
  return 0;
}
