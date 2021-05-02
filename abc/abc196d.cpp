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

// void dfs(vector<vector<bool>> g, int a, int b) {
//   int h = g.size();
//   int w = g[0].size();
//   if(a > 0) {
//     for(int i=0;i<h;i++) {
//       for(int j=0;j<w;j++) {

//       }
//     }
//   }
// }

ll dfs(vector<vector<bool>> ck) {
  ll res = 0;
  int h = ck.size();
  int w = ck[0].size();
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(!ck[i][j]) {
        ck[i][j] = true;
        if(i-1 >= 0 && !ck[i-1][j]) {
          ck[i-1][j] = true;
          res += dfs(ck);
          ck[i-1][j] = false;
        }
        if(i+1 < h && !ck[i+1][j]) {
          ck[i+1][j] = true;
          res += dfs(ck);
          ck[i+1][j] = false;
        }
        if(j-1 >= 0 && !ck[i][j-1]) {
          ck[i][j-1] = true;
          res += dfs(ck);
          ck[i][j-1] = false;
        }
        if(j+1 < w && !ck[i][j+1]) {
          ck[i][j+1] = true;
          res += dfs(ck);
          ck[i][j+1] = false;
        }
        return res;
      }
    }
  }
  return 1;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  int a, b;
  cin >> a >> b;
  int t = h*w;
  ll ans = 0;
  for(int i=0;i<(1<<t);i++) {
    int l = 0;
    vector<vector<bool>> ck(h, vector<bool>(w, false));
    vector<vector<int>> g(h, vector<int>(w, 0));
    for(int j=0;j<t;j++) {
      if(i & (1<<j)) {
        g[j/w][j%w] = 1;
        ck[j/w][j%w] = true;
        l++;
      }
    }
    if(l != b) continue;
    ll res = dfs(ck);
    ans += res;
  }
  cout << ans << endl;
  // vector<vector<vector<ll>>> dp((1 << t), vector<vector<ll>>(a+1, vector<ll>(b+1, 0)));
  // dp[0][a][b] = 1;
  // for(int i=0;i<(1<<t);i++) {
  //   for(int j=a;j>=0;j--) {
  //     for(int k=b;k>=0;k--) {
  //       for(int l=0;l<t;l++) {
  //         if(i & (1 << l)) continue;
  //         int ni = i | (1<<l);
  //         if(k-1 >= 0) dp[ni][j][k-1] += dp[i][j][k];
  //         if(j-1 < 0) continue;
  //         if(l != 0 && l%w != 0 && !(i & (1<<(l-1)))) {
  //           ni = i | (1<<l) | (1<<(l-1));
  //           dp[ni][j-1][k] += dp[i][j][k];
  //         }
  //         if(l != t-1 && l%w != w-1 && !(i & (1<<(l+1)))) {
  //           ni = i | (1<<l) | (1<<(l+1));
  //           dp[ni][j-1][k] += dp[i][j][k];
  //         }
  //         if(l+w < t && !(i & (1<<(l+w)))) {
  //           ni = i | (1<<l) | (1<<(l+w));
  //           dp[ni][j-1][k] += dp[i][j][k];
  //         }
  //         if(l-w >= 0 && !(i & (1<<(l-w)))) {
  //           ni = i | (1<<l) | (1<<(l-w));
  //           dp[ni][j-1][k] += dp[i][j][k];
  //         }
  //       }
  //     }
  //   }
  // }
  // ll ans = dp[(1<<t)-1][0][0];
  // cout << ans << endl;
  return 0;
}
