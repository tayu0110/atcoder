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
template<class T>
struct heap {
  priority_queue<T, vector<T>, greater<T>> pq;
  heap() : pq() {}
  heap(priority_queue<T, vector<T>, greater<T>> pq) : pq(pq) {}
  void push(T c) { pq.push(c); }
  T top() { return pq.top(); }
  void pop() { pq.pop(); }
  bool empty() { return pq.empty(); }
  int size() { return pq.size(); }
  void swap(heap<T> nt) { pq.swap(nt.pq); }
};
using P = pair<int, pii>;
class JumpingTiger {
  bool check(int width, pii now, pii dir, vector<string> &p) {
    int h = p.size();
    int w = p[0].size();
    int ny = now.first;
    int nx = now.second;
    int dy = dir.first;
    int dx = dir.second;
    if(ny + dy < 0 || ny + dy >= h || nx + dx < 0 || nx + dx >= w) return false;
    if(p[ny+dy][nx+dx] == '.' || p[ny+dy][nx+dx] == 'L') return true;
    int cnt = 0;
    while(cnt < width && ny+dy >= 0 && ny+dy < h && nx + dx >= 0 && nx + dx < w) {
      ny += dy;
      nx += dx;
      cnt++;
      if(p[ny][nx] == '#') continue;
      bool f = check(cnt, make_pair(ny, nx), dir, p);
      if(f) return f;
    }
    return false;
  }
public:
  int travel(vector<string> p) {
    int h = p.size();
    int w = p[0].length();
    pii start = {-1, -1};
    pii goal = {-1, -1};
    for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
      if(p[i][j] == 'T') start = {i, j};
      if(p[i][j] == 'L') goal = {i, j};
    }
    vector<vector<int>> ck(h, vector<int>(w, -1));
    heap<P> nt;
    nt.push({0, start});
    while(!nt.empty()) {
      P _ = nt.top();
      nt.pop();
      pii now = _.second;
      int d = _.first;
      int y = now.first;
      int x = now.second;
      if(ck[y][x] >= 0) continue;
      ck[y][x] = d;
      for(int r=0;r<h;r++) {
        if(p[r][x] == '#') continue;
        if(ck[r][x] >= 0) continue;
        if(abs(r-y) > 1) {
          if(r < y && !check(y - r, make_pair(r, x), make_pair(-1, 0), p)) continue;
          else if(r > y && !check(r - y, make_pair(r, x), make_pair(1, 0), p)) continue;
        }
        nt.push({d+1, make_pair(r, x)});
      }
      for(int c=0;c<w;c++) {
        if(p[y][c] == '#') continue;
        if(ck[y][c] >= 0) continue;
        if(abs(x-c) > 1) {
          if(c < x && !check(x - c, make_pair(y, c), make_pair(0, -1), p)) continue;
          else if(c > x && !check(c - x, make_pair(y, c), make_pair(0, 1), p)) continue;
        }
        nt.push({d+1, make_pair(y, c)});
      }
    }
    return ck[goal.first][goal.second];
  }
};

int main() {
  cout << (int)' ' << endl;
  return 0;
}