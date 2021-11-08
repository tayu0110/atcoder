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
ll solve(string s) {
  int n = s.length();
  vector<vector<ll>> dp(n+1, vector<ll>(2, 0));
  dp[0][0] = 1;
  for(int i=0;i<n;i++) {
    int num = s[i] - '0';
    for(int j=0;j<2;j++) {
      for(int k=0;k<10;k++) {
        if(k == 4 || k == 9) continue;
        if(j == 0 && k > num) continue;
        if(k < num) dp[i+1][1] += dp[i][j];
        else dp[i+1][j] += dp[i][j];
      }
    }
  }
  return dp[n][0] + dp[n][1];
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll a, b;
  cin >> a >> b;
  a--;
  string s = to_string(a), t = to_string(b);
  ll ans = b-a+1 - (solve(t) - solve(s)) - 1;
  cout << ans << endl;
  return 0;
}
