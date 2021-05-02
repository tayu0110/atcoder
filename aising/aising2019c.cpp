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
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  for(int i=0;i<h;i++) {
    cin >> s[i];
  }
  vector<vector<bool>> ck(h, vector<bool>(w, false));
  ll ans = 0;
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(ck[i][j]) continue;
      queue<pii> nt;
      ll b = 0, wh = 0;
      nt.push({i, j});
      while(!nt.empty()) {
        int y = nt.front().first;
        int x = nt.front().second;
        nt.pop();
        if(ck[y][x]) continue;
        if(s[y][x] == '.') wh++;
        else b++;
        ck[y][x] = true;
        if(y-1 >= 0 && !ck[y-1][x] && s[y][x] != s[y-1][x]) {
          nt.push({y-1, x});
        }
        if(y+1 < h && !ck[y+1][x] && s[y][x] != s[y+1][x]) {
          nt.push({y+1, x});
        }
        if(x-1 >= 0 && !ck[y][x-1] && s[y][x-1] != s[y][x]) {
          nt.push({y, x-1});
        }
        if(x+1 < w && !ck[y][x+1] && s[y][x+1] != s[y][x]) {
          nt.push({y, x+1});
        }
      }
      ans += wh * b;
    }
  }
  cout << ans << endl;
  return 0;
}
