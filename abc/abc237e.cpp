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
  int n, m;
  cin >> n >> m;
  vector<ll> h(n);
  for(int i=0;i<n;i++) cin >> h[i];
  Graph t(n);
  for(int i=0;i<m;i++) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    t[u].push_back(v);
    t[v].push_back(u);
  }
  heap<pll> nt;
  nt.push({0, 0});
  vector<ll> fun(n, INF);
  while(!nt.empty()) {
    auto [d, now] = nt.top();
    nt.pop();
    if(fun[now] <= d) continue;
    fun[now] = d;
    for(auto to : t[now]) {
      if(h[now] > h[to]) {
        nt.push({d - (h[now] - h[to]), to});
      } else if(h[now] < h[to]) {
        nt.push({d + (h[to] - h[now]) * 2, to});
      } else {
        nt.push({d, to});
      }
    }
  }
  ll mn = *min_element(fun.begin(), fun.end());
  if(mn > 0) cout << 0 << endl;
  else cout << -mn << endl;
  return 0;
}
