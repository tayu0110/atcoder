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
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
vector<bool> ck;
bool f = true;
int dfs(int now, vector<int> &d, Graph &t) {
  int res = 0;
  for(auto e : t[now]) {
    if(ck[e]) {
      f = false;
      return -1;
    }
    if(d[e] >= 0) res = max(res, d[e]);
    else {
      ck[now] = true;
      res = max(res, dfs(e, d, t));
      ck[now] = false;
    }
  }
  return d[now] = res + 1;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<int>> a(n, vector<int>(n-1));
  int cnt = 0;
  map<pii, int> mp;
  Graph t(n*n);
  for(int i=0;i<n;i++) for(int j=0;j<n-1;j++) cin >> a[i][j], a[i][j]--;
  for(int i=0;i<n;i++) for(int j=0;j<n-1;j++) {
    int k = a[i][j];
    pii l = {min(i, k), max(i, k)};
    if(mp.find(l) == mp.end()) {
      mp[l] = cnt;
      cnt++;
    }
    if(j == 0) continue;
    pii p = {min(i, a[i][j-1]), max(i, a[i][j-1])};
    t[mp[p]].push_back(mp[l]);
  }
  vector<int> d(n*n, -1);
  ck.assign(n*n, false);
  for(int i=0;i<cnt;i++) {
    if(d[i] >= 0) continue;
    dfs(i, d, t);
  }
  if(!f) {
    cout << -1 << endl;
    return 0;
  }
  int ans = *max_element(d.begin(), d.end());
  cout << ans << endl;
  return 0;
}
