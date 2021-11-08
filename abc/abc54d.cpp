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

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, ma, mb;
  cin >> n >> ma >> mb;
  vector<pair<pii, int>> p(n);
  int sa = 0, sb = 0;
  for(int i=0;i<n;i++) {
    int a, b, c;
    cin >> a >> b >> c;
    p[i] = {{a, b}, c};
    sa += a;
    sb += b;
  }
  vector<vector<int>> dp(sa+1, vector<int>(sb+1, inf));
  dp[0][0] = 0;
  for(int k=0;k<n;k++) {
    int a = p[k].first.first, b = p[k].first.second, c = p[k].second;
    for(int i=sa;i>=0;i--) for(int j=sb;j>=0;j--) {
      if(i+a > sa || j+b > sb) continue;
      dp[i+a][j+b] = min(dp[i+a][j+b], dp[i][j]+c);
    }
  }
  int cnt = 1;
  int ans = inf;
  while(ma*cnt < sa && mb*cnt < sb) {
    int a = ma*cnt, b = mb*cnt;
    ans = min(ans, dp[a][b]);
    cnt++;
  }
  if(ans == inf) ans = -1;
  cout << ans << endl;
  return 0;
}
