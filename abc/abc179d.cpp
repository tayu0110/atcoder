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
  ll n, k;
  cin >> n >> k;
  vector<pll> lr(k);
  for(int i=0;i<k;i++) {
    cin >> lr[i].first >> lr[i].second;
    lr[i].second++;
  }
  vector<ll> dp(2*n+1, 0);
  dp[1] = 1;
  dp[2] = -1;
  for(int i=1;i<n+1;i++) {
    dp[i] += dp[i-1];
    dp[i] %= 998244353;
    for(int j=0;j<k;j++) {
      ll l = lr[j].first;
      ll r = lr[j].second;
      dp[i+l] += dp[i];
      dp[i+r] -= dp[i];
    }
  }
  if(dp[n] < 0) dp[n] += 998244353;
  cout << dp[n] << endl;
  return 0;
}
