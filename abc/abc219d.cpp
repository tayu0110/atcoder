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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, x, y;
  cin >> n >> x >> y;
  vector<pii> p(n);
  for(int i=0;i<n;i++) {
    int a, b;
    cin >> a >> b;
    p[i] = {a, b};
  }
  vector<vector<int>> dp(400, vector<int>(400, inf));
  dp[0][0] = 0;
  for(int i=0;i<n;i++) {
    int a = p[i].first, b = p[i].second;
    for(int j=x;j>=0;j--) for(int k=y;k>=0;k--) {
      if(dp[j][k] == inf) continue;
      int nj = min(x, j+a);
      int nk = min(y, k+b);
      dp[nj][nk] = min(dp[nj][nk], dp[j][k] + 1);
    }
  }
  int ans = inf;
  for(int i=x;i<dp.size();i++) for(int j=y;j<dp[0].size();j++) {
    ans = min(ans, dp[i][j]);
  }
  if(ans == inf) cout << -1 << endl;
  else cout << ans << endl;
  return 0;
}
