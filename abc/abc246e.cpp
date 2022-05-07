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
int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  int sx, sy, tx, ty;
  cin >> sx >> sy >> tx >> ty;
  sx--; sy--; tx--; ty--;
  vector<string> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  heap<tuple<int, int, int>> nt;
  nt.push({0, sx, sy});
  vector<int> dx = {1, 1, -1, -1};
  vector<int> dy = {1, -1, 1, -1};
  vector<vector<int>> d(n, vector<int>(n, inf));
  while(!nt.empty()) {
    auto [nd, x, y] = nt.top();
    nt.pop();
    if(d[x][y] < nd) continue;
    for(int i=0;i<4;i++) {
      int nx = x + dx[i], ny = y + dy[i];
      while(0 <= nx && nx < n && 0 <= ny && ny < n) {
        if(s[nx][ny] == '#') break;
        if(d[nx][ny] < nd+1) break;
        if(d[nx][ny] == nd+1) {
          nx += dx[i];
          ny += dy[i];
          continue;
        }
        nt.push({nd+1, nx, ny});
        d[nx][ny] = nd+1;
        nx += dx[i];
        ny += dy[i];
      }
    }
  }
  if(d[tx][ty] == inf) d[tx][ty] = -1;
  cout << d[tx][ty] << endl;
  return 0;
}
