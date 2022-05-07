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
};
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<tuple<double, double, double, double>> p(n);
  for(int i=0;i<n;i++) {
    int x, y, t, r;
    cin >> x >> y >> t >> r;
    p[i] = {x, y, t, r};
  }
  // vector<double> dist(n, 1e100);
  // heap<pair<double, int>> nt;
  // nt.push({0, 0});
  // vector<double> res;
  // while(!nt.empty()) {
  //   auto [d, now] = nt.top();
  //   nt.pop();
  //   if(dist[now] <= d) continue;
  //   dist[now] = d;
  //   res.push_back(d);
  //   auto [x, y, t, r] = p[now];
  //   for(int i=0;i<n;i++) {
  //     if(i == now) continue;
  //     auto [tx, ty, tt, tr] = p[i];
  //     double nd = d + hypot(abs(x - tx), abs(y - ty)) / min(t, tr);
  //     nt.push({nd, i});
  //   }
  // }
  // sort(res.begin(), res.end(), greater<double>());
  // res.pop_back();
  // double ans = 0;
  // for(int i=0;i<res.size();i++) ans = max(ans, res[i] + i);
  // cout << ans << endl;
  vector<vector<double>> dp(n, vector<double>(n, INF));
  for(int i=0;i<n;i++) {
    auto [x, y, t, r] = p[i];
    for(int j=i;j<n;j++) {
      if(i == j) {
        dp[i][i] = 0;
        continue;
      }
      auto [tx, ty, tt, tr] = p[j];
      dp[i][j] = hypot(abs(x-tx), abs(y-ty)) / min(t, tr);
      dp[j][i] = hypot(abs(x-tx), abs(y-ty)) / min(tt, r);
    }
  }
  for(int k=0;k<n;k++) for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    dp[i][j] = min(dp[i][j], dp[i][k] + dp[k][j]);
  }
  vector<double> d(n);
  for(int i=0;i<n;i++) d[i] = dp[0][i];
  sort(d.begin(), d.end(), greater<double>());
  d.pop_back();
  for(int i=0;i<d.size();i++) d[i] += i;
  cout << *max_element(d.begin(), d.end()) << endl;
  return 0;
}
