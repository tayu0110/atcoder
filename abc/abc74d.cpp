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
  int n;
  cin >> n;
  vector<vector<ll>> a(n, vector<ll>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) cin >> a[i][j];
  vector<vector<ll>> dp = a;
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) if(dp[i][j] == 0) dp[i][j] = INF;
  for(int k=0;k<n;k++) {
    for(int i=0;i<n;i++) {
      for(int j=0;j<n;j++) {
        if(dp[i][j] == INF) continue;
        if(dp[i][k] + dp[k][j] < dp[i][j]) {
          cout << -1 << endl;
          return 0;
        }
        if(dp[i][k] + dp[k][j] == dp[i][j]) {
          // DEBUG(i);DEBUG(j);DEBUG_EN(k);
          a[i][j] = a[j][i] = 0;
        }
      }
    }
  }
  ll ans = 0;
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    ans += a[i][j];
  }
  ans /= 2;
  cout << ans << endl;
  return 0;
}
