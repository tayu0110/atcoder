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
  pair<int, int> to;
  long long weight;
  Edge() : to({0, 0}), weight(0) {}
  Edge(pair<int, int> to, long long weight) : to(to), weight(weight) {}
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
using heap = priority_queue<Edge, vector<Edge>, greater<Edge>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  ll t;
  cin >> h >> w >> t;
  vector<string> s(h);
  for(int i=0;i<h;i++) cin >> s[i];
  int sx, sy, gx, gy;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    if(s[i][j] == 'S') {
      sx = j; sy = i;
    } else if(s[i][j] == 'G') {
      gx = j, gy = i;
    }
  }
  ll l = 1, r = 1001001001;
  vector<int> dx = {-1, 0, 1, 0};
  vector<int> dy = {0, -1, 0, 1};
  while(r-l > 1) {
    ll m = (r+l) / 2;
    heap nt;
    nt.push({{sy, sx}, 0});
    vector<vector<ll>> d(h, vector<ll>(w, -1));
    while(!nt.empty()) {
      auto now = nt.top();
      nt.pop();
      int y = now.to.first;
      int x = now.to.second;
      ll nd = now.weight;
      if(d[y][x] >= 0) continue;
      d[y][x] = nd;
      for(int i=0;i<4;i++) {
        int nx = x + dx[i];
        int ny = y + dy[i];
        if(nx < 0 || nx >= w || ny < 0 || ny >= h) continue;
        if(d[ny][nx] >= 0) continue;
        if(s[ny][nx] == '#') nt.push({{ny, nx}, nd+m});
        else nt.push({{ny, nx}, nd+1});
      }
    }
    if(d[gy][gx] <= t) l = m;
    else r = m;
  }
  cout << l << endl;
  return 0;
}
