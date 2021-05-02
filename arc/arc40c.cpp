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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<string> s(n);
  for(int i=0;i<n;i++) {
    cin >> s[i];
  }
  vector<vector<int>> g(n, vector<int>(n, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(j == 0) {
        if(s[i][j] == '.') g[i][j] = 1;
        else g[i][j] = 0;
      } else {
        if(s[i][j] == '.') g[i][j] = g[i][j-1] + 1;
        else g[i][j] = g[i][j-1];
      }
    }
  }
  int ans = 0;
  for(int i=0;i<n;i++) {
    if(g[i][n-1] == 0) continue;
    for(int j=0;j<n;j++) {
      if(g[i][j] == g[i][n-1]) {
        ans++;
        if(i+1 < n) {
          int k = i + 1;
          if(s[k][j] == '.') g[k][j]--;
          for(int l=j+1;l<n;l++) {
            g[k][l] = g[k][l-1];
          }
        }
        break;
      }
    }
  }
  cout << ans << endl;
  return 0;
}
