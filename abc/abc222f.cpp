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
struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};
using weightedGraph = vector<vector<Edge>>;
vector<vector<ll>> memo;
vector<ll> d;
ll dfs(int now, int par, int nd, weightedGraph &t) {
  int cnt = -1;
  ll res = 0;
  for(auto [to, w] : t[now]) {
    cnt++;
    if(to == par) continue;
    memo[now][cnt] = max(dfs(to, now, nd+w, t), nd+d[now]);
    res = max(res, memo[now][cnt]);
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  weightedGraph t(n);
  memo.assign(n, vector<ll>(0));
  for(int i=0;i<n-1;i++) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--;b--;
    t[a].push_back(Edge(b, c));
    t[b].push_back(Edge(a, c));
    memo[a].push_back(-1);
    memo[b].push_back(-1);
  }
  d.assign(n, -1);
  for(int i=0;i<n;i++) cin >> d[i];
  dfs(0, -1, 0, t);
  
  return 0;
}
