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
void dfs(int now, int dep, int &ord, Graph &t, vector<vector<int>> &d, vector<int> &in, vector<int>&out) {
  in[now] = ord;
  d[dep].push_back(ord);
  for(auto e : t[now]) {
    ord++;
    dfs(e, dep+1, ord, t, d, in, out);
  }
  ord++;
  out[now] = ord;
  return;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  Graph t(n);
  for(int i=1;i<n;i++) {
    int k;
    cin >> k;
    k--;
    t[k].push_back(i);
  }
  vector<vector<int>> dep(n+1); 
  vector<int> in(n), out(n);
  int ord = 0;
  dfs(0, 0, ord, t, dep, in, out);
  int q;
  cin >> q;
  while(q--) {
    int u, d;
    cin >> u >> d;
    u--;
    int i = in[u], o = out[u];
    auto iit = lower_bound(dep[d].begin(), dep[d].end(), i);
    auto oit = upper_bound(dep[d].begin(), dep[d].end(), o);
    int res = oit - iit;
    cout << res << endl;
  }
  return 0;
}
