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
  int n;
  cin >> n;
  string s;
  cin >> s;
  string c[4] = {"A", "B", "X", "Y"};
  int ans = n;
  for(int i=0;i<4;i++) {
    for(int j=0;j<4;j++) {
      for(int k=0;k<4;k++) {
        for(int l=0;l<4;l++) {
          string L = c[i] + c[j];
          string R = c[k] + c[l];
          vector<vector<int>> dp(2, vector<int>(n, n));
          for(int m=0;m<n;m++) {
            if(m == 0) {
              dp[0][m] = 0;
              dp[1][m] = 1;
              continue;
            }
            dp[0][m] = min(dp[1][m-1], dp[0][m-1] + 1);
            if((s[m-1] == L[0] && s[m] == L[1]) || (s[m-1] == R[0] && s[m] == R[1])) {
              dp[1][m] = min(dp[1][m-1]+1, dp[0][m-1]+1);
            } else {
              dp[1][m] = min(dp[1][m-1]+1, dp[0][m-1]+2);
            }
          }
          ans = min(ans, min(dp[1][n-1], dp[0][n-1]+1));
        }
      }
    }
  }
  cout << ans << endl;
  return 0;
}
