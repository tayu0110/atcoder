#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<string> c(h);
  for(int i=0;i<h;i++) cin >> c[i];
  vector<vector<int>> d(h, vector<int>(w, -1));
  queue<tuple<int, int, int>> nt;
  nt.push({0, 0, 0});
  vector<int> dx = {0, 1};
  vector<int> dy = {1, 0};
  while(!nt.empty()) {
    auto [y, x, dist] = nt.front();
    nt.pop();
    if(d[y][x] >= 0) continue;
    d[y][x] = dist;
    for(int i=0;i<4;i++) {
      int nx = x + dx[i];
      int ny = y + dy[i];
      if(nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
      if(c[ny][nx] == '#') continue;
      nt.push({ny, nx, dist+1});
    }
  }
  int mx = 0;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) mx = max(mx, d[i][j]);
  cout << mx + 1 << endl;
  return 0;
}
