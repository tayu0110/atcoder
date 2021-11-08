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
using heap = priority_queue<pair<int, pii>, vector<pair<int, pii>>, greater<pair<int, pii>>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<int> dx = {1, 0, -1, 0}, dy = {0, 1, 0, -1};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  for(int i=0;i<h;i++) cin >> s[i];
  vector<vector<int>> d(h, vector<int>(w, -1));
  heap nt;
  nt.push({0, {0, 0}});
  auto f = [&](int r, int c) {
    return r < 0 || r >= h || c < 0 || c >= w;
  };
  while(!nt.empty()) {
    auto p = nt.top();
    int dist = p.first;
    int y = p.second.first;
    int x = p.second.second;
    nt.pop();
    if(d[y][x] >= 0) continue;
    d[y][x] = dist;
    if(s[y][x] == '.') {
      for(int i=0;i<4;i++) {
        int ny = y + dy[i];
        int nx = x + dx[i];
        if(!f(ny, nx) && d[ny][nx] < 0) nt.push({dist, {ny, nx}});
      }
    } else {
      for(int i=-1;i<2;i++) for(int j=-1;j<2;j++) {
        if(i == 0 && j == 0) continue;
        int ny = y + i;
        int nx = x + j;
        if(!f(ny, nx) && s[ny][nx] == '.' && d[ny][nx] < 0) nt.push({dist+1, {ny, nx}});
      }
      for(int i=-1;i<2;i++) {
        int ny = y+2;
        int nx = x+i;
        if(!f(ny, nx) && d[ny][nx] < 0) nt.push({dist+1, {ny, nx}});
        ny = y-2;
        if(!f(ny, nx) && d[ny][nx] < 0) nt.push({dist+1, {ny, nx}});
        ny = y+i;
        nx = x+2;
        if(!f(ny, nx) && d[ny][nx] < 0) nt.push({dist+1, {ny, nx}});
        nx = x-2;
        if(!f(ny, nx) && d[ny][nx] < 0) nt.push({dist+1, {ny, nx}});
      }
    }
  }
  cout << d[h-1][w-1] << endl;
  return 0;
}
