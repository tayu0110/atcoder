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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w, k;
  cin >> h >> w >> k;
  vector<string> a(h);
  for(int i=0;i<h;i++) cin >> a[i];
  int s = -1;
  auto f = [w](int r, int c) { return r * w + c; };
  auto unf = [w](int now) { return make_pair(now / w, now % w); };
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    if(a[i][j] == 'S') s = f(i, j);
  }
  vector<int> dx = {0, 1, 0, -1};
  vector<int> dy = {1, 0, -1, 0};
  vector<int> dist(h*w, -1);
  queue<pii> nt;
  nt.push({s, 0});
  while(!nt.empty()) {
    auto [now, d] = nt.front();
    nt.pop();
    if(dist[now] >= 0) continue;
    dist[now] = d;
    if(d == k) continue;
    auto [r, c] = unf(now);
    for(int i=0;i<4;i++) {
      int y = r + dy[i];
      int x = c + dx[i];
      if(y < 0 || y >= h || x < 0 || x >= w) continue;
      if(a[y][x] == '#') continue;
      if(dist[f(y, x)] >= 0) continue;
      nt.push({f(y, x), d+1});
    }
  }
  int ans = inf;
  vector<int> v = {0, h-1}, t = {0, w-1};
  for(int i=0;i<h*w;i++) {
    if(dist[i] < 0) continue;
    auto [r, c] = unf(i);
    for(int j=0;j<2;j++) {
      int d = abs(v[j] - r);
      if(!d) ans = 0;
      ans = min(ans, (d-1) / k + 1);
    }
    for(int j=0;j<2;j++) {
      int d = abs(t[j] - c);
      if(!d) ans = 0;
      ans = min(ans, (d-1) / k + 1);
    }
  }
  cout << ans+1 << endl;
  return 0;
}
