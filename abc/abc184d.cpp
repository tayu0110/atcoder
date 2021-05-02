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

double dp[101][101][101];
double dfs(int a, int b, int c) {
  if(a==100 || b==100 || c==100) return 0;
  if(dp[a][b][c]) return dp[a][b][c];
  double t = a + b + c;
  double x = a, y = b, z = c;
  dp[a][b][c] += x / t * (dfs(a+1, b, c) + 1) + y / t * (dfs(a, b+1, c) + 1) + z / t * (dfs(a, b, c+1) + 1);
  return dp[a][b][c];
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int a, b, c;
  cin >> a >> b >> c;
  double res = dfs(a, b, c);
  cout << res << endl;
  return 0;
}
