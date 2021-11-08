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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 62;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<pll> p(n);
  set<ll> x, y;
  for(int i=0;i<n;i++) {
    cin >> p[i].first >> p[i].second;
    x.insert(p[i].first);
    y.insert(p[i].second);
  }
  int ys = y.size(), xs = x.size();
  map<ll, int> mx, my;
  vector<ll> px(xs), py(ys);
  int cnt = 0;
  for(auto e : x) mx[e] = cnt, px[cnt] = e, cnt++;
  cnt = 0;
  for(auto e : y) my[e] = cnt, py[cnt] = e, cnt++;
  vector<vector<int>> ts(ys+1, vector<int>(xs+1));
  for(int i=0;i<n;i++) ts[my[p[i].second]+1][mx[p[i].first]+1]++;
  for(int i=1;i<ys+1;i++) for(int j=1;j<xs+1;j++) ts[i][j] += ts[i][j-1];
  for(int i=1;i<ys+1;i++) for(int j=1;j<xs+1;j++) ts[i][j] += ts[i-1][j];
  ll ans = INF;
  for(int i=1;i<ys+1;i++) for(int j=1;j<xs+1;j++) {
    for(int s=0;s<i;s++) for(int t=0;t<j;t++) {
      int nk = ts[i][j] - ts[i][t] - ts[s][j] + ts[s][t];
      if(nk < k) continue;
      ll ns = abs(py[i-1] - py[s]) * abs(px[j-1] - px[t]);
      // DEBUG(py[i-1]);DEBUG(py[s]);DEBUG(px[i-1]);DEBUG_EN(px[t]);
      // DEBUG_EN(ns);
      ans = min(ans, ns);
    }
  }
  cout << ans << endl;
  return 0;
}
