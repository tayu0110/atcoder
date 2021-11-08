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
  int n;
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  map<int, int> mp;
  for(int i=0;i<n;i++) mp[a[i]]++;
  const int m = 310;
  vector<vector<vector<double>>> dp(m, vector<vector<double>>(m, vector<double>(m, 0)));
  for(int k=0;k<n+1;k++) for(int j=0;j<n+1;j++) for(int i=0;i<n+1;i++) {
    int sum = i+j+k;
    if(!sum) continue;
    dp[i][j][k] = 1.0 * n / sum;
    if(i) dp[i][j][k] += dp[i-1][j][k] * i / sum;
    if(j) dp[i][j][k] += dp[i+1][j-1][k] * j / sum;
    if(k) dp[i][j][k] += dp[i][j+1][k-1] * k / sum;
  }
  cout << dp[mp[1]][mp[2]][mp[3]] << endl;
  return 0;
}
