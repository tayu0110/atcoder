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
  int n, m;
  ll l;
  cin >> n >> m >> l;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--;b--;
    t[a].push_back(Edge(b, c));
    t[b].push_back(Edge(a, c));
  }
  vector<vector<pll>> d(n, vector<pll>(n, {INF, INF}));
  // heap<pair<pll, int>> nt;
  for(int i=0;i<n;i++) {
    vector<bool> ck(n, false);
    d[i][i] = {0, 0};
    for(int j=0;j<n;j++) {
      pll mn = {INF, INF};
      int idx = 0;
      for(int k=0;k<n;k++) {
        if(ck[k]) continue;
        if(mn > d[i][k]) {
          mn = d[i][k];
          idx = k;
        }
      }
      ck[idx] = true;
      ll oil = l - mn.second;
      for(auto e : t[idx]) {
        int to = e.to;
        if(ck[to]) continue;
        ll w = e.weight;
        if(l-w < 0) continue;
        d[i][to] = min(d[i][to], {mn.first+1, w});
        if(oil-w < 0) continue;
        d[i][to] = min(d[i][to], {mn.first, l-(oil-w)});
      }
    }
    // nt.push({{0, 0}, i});
    // while(!nt.empty()) {
    //   auto p = nt.top();
    //   int s = p.first.first;
    //   ll rest = p.first.second;
    //   int now = p.second;
    //   nt.pop();
    //   if(d[i][now] >= 0) continue;
    //   d[i][now] = s;
    //   ll oil = l-rest;
    //   for(auto e : t[now]) {
    //     int to = e.to;
    //     ll w = e.weight;
    //     if(d[i][to] >= 0) continue;
    //     if(l-w < 0) continue;
    //     nt.push({{s+1, w}, to});
    //     if(oil-w < 0) continue;
    //     nt.push({{s, l-(oil-w)}, to});
    //   }
    // }
  }
  int q;
  cin >> q;
  vector<ll> res;
  while(q--) {
    int u, v;
    cin >> u >> v;
    u--;v--;
    ll r = d[u][v].first;
    if(r == INF) r = -1;
    res.push_back(r);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
