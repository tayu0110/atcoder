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
  int n;
  cin >> n;
  string s, x;
  cin >> s >> x;
  vector<vector<int>> dp(7, vector<int>(n+1, 0));
  dp[0][0] = 1;
  for(int i=0;i<n;i++) {
    for(int j=0;j<7;j++) {
      if(!dp[j][i]) continue;
      int k = j*10 + (s[i] - '0');
      dp[k%7][i+1]++;
      int l = j*10;
      dp[l%7][i+1]++;
    }
  }
  cout << dp[0][n] << endl;
  if(dp[0][n] == 0) {
    cout << "Aoki" << endl;
    return 0;
  }
  if(x[n-1] == 'T') {
    cout << "Takahashi" << endl;
    return 0;
  } else {
    cout << "Aoki" << endl;
    return 0;
  }
  return 0;
}
