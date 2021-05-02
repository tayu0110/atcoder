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
  int r, c, d;
  cin >> r >> c >> d;
  vector<vector<int>> a(r, vector<int>(c, 0));
  for(int i=0;i<r;i++){
    for(int j=0;j<c;j++){
      cin >> a[i][j];
    }
  }
  int ans = 0;
  queue<pii> nt;
  nt.push(make_pair(0, 0));
  vector<vector<int>> dist(r, vector<int>(c,-1));
  dist[0][0] = 0;
  while(!nt.empty()) {
    int y = nt.front().first;
    int x = nt.front().second;
    nt.pop();
    int t = dist[y][x];
    if(d % 2 == t % 2) ans = max(ans, a[y][x]);
    if(t == d) continue;
    if(y-1 >= 0 && dist[y-1][x] == -1) {
      dist[y-1][x] = t+1;
      nt.push(make_pair(y-1, x));
    }
    if(y+1 < r && dist[y+1][x] == -1) {
      dist[y+1][x] = t+1;
      nt.push(make_pair(y+1, x));
    }
    if(x-1 >= 0 && dist[y][x-1] == -1) {
      dist[y][x-1] = t+1;
      nt.push(make_pair(y, x-1));
    }
    if(x+1 < c && dist[y][x+1] == -1) {
      dist[y][x+1] = t+1;
      nt.push(make_pair(y, x+1));
    }
  }
  cout << ans << endl;
  return 0;
}
