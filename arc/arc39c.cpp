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

using namespace std;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);
const int U = 0, D = 1, L = 2, R = 3;
struct Point {
  int x, y;
  int neighbor[4];
  Point() {}
  Point(int x, int y) : x(x), y(y), neighbor{y+1, y-1, x-1, x+1} {}
  bool operator<(const Point &r) { return x == r.x ? y < r.y : x < r.x; }
  bool operator==(const Point &r) { return x == r.x && y == r.y; }
};
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int k;
  string s;
  cin >> k >> s;
  map<pii, Point> mp;
  mp[{0, 0}] = Point(0, 0);
  int x = 0, y = 0;
  int dx[] = {0, 1, 0, -1};
  int dy[] = {1, 0, -1, 0};
  for(int i=0;i<k;i++) {
    char c = s[i];
    // DEBUG(x);DEBUG_EN(y);
    Point now = mp[{x, y}];
    if(c == 'L') x = now.neighbor[L];
    else if(c == 'R') x = now.neighbor[R];
    else if(c == 'U') y = now.neighbor[U];
    else y = now.neighbor[D];
    Point nt[4];
    for(int j=0;j<4;j++) {
      int nx, ny;
      if(j < 2) nx = now.x, ny = now.neighbor[j];
      else nx = now.neighbor[j], ny = now.y;
      if(mp.find({nx, ny}) == mp.end()) mp[{nx, ny}] = nt[j] = Point(nx, ny);
      else nt[j] = mp[{nx, ny}];
    }
    nt[U].neighbor[D] = nt[D].y;
    nt[D].neighbor[U] = nt[U].y;
    nt[L].neighbor[R] = nt[R].x;
    nt[R].neighbor[L] = nt[L].x;
    for(int j=0;j<4;j++) {
      int tx = nt[j].x, ty = nt[j].y;
      Point now = nt[j];
      // DEBUG(tx);DEBUG(ty);DEBUG(now.neighbor[U]);DEBUG(now.neighbor[D]);DEBUG(now.neighbor[L]);DEBUG_EN(now.neighbor[R]);
      mp[{tx, ty}] = nt[j];
    }
  }
  printf("%d %d\n", x, y);
  return 0;
}
