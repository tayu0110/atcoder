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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

template<class Cap, class Cost>
struct MinCostFlow {
private:
  int n;
  struct edge {
    int from, to;
    Cap cap, flow;
    Cost cost;
  };
  struct inside_edge {
    int to, rev;
    Cap cap;
    Cost cost;
  };
  vector<edge> edges;
public:
  MinCostFlow() {}
  MinCostFlow(int n) : n(n) {}
  int add_edge(int from, int to, Cap cap, Cost cost) {
    int m = edges.size();
    edges.push_back({from, to, cap, 0, cost});
    return m;
  }
  edge get_edge(int i) {
    return edges[i];
  }
  pair<Cap, Cost> flow(int s, int t) {
    return flow(s, t, numeric_limits<Cap>::max());
  }
  pair<Cap, Cost> flow(int s, int t, Cap flow_limit) {
    return slope(s, t, flow_limit).back();
  }
  vector<pair<Cap, Cost>> slope(int s, int t) {
    return slope(s, t, numeric_limits<Cap>::max());
  }
  vector<pair<Cap, Cost>> slope(int s, int t, Cap flow_limit) {
    int m = edges.size();
    vector<int> edge_idx(m);
    auto g = [&]() {
      vector<int> degree(n), redge_idx(m);
      vector<pair<int, inside_edge>> elist;
      elist.reserve(2*m);
      for(int i = 0; i < m; i++) {
        edge e = edges[i];
        edge_idx[i] = degree[e.from]++;
        redge_idx[i] += degree[e.to]++;
        elist.push_back({e.from, {e.to, -1, e.cap - e.flow, e.cost}});
        elist.push_back({e.to, {e.from, -1, e.flow, -e.cost}});
      }
      auto _g = 
    }
  }
};

struct LazySegmentTree {
 private:
  int n;
  vector<ll> node, lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.resize(2*n-1); lazy.resize(2*n-1);
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  LazySegmentTree(vector<ll> &v) {
    int sz = v.size();
    init(sz);
    for(int i = 0; i < sz; i++) node[i+n-1] = v[i];
    // RSQ initialize
    for(int i = n-2; i >= 0; i--) node[i] = node[i*2+1] + node[i*2+2];
    // RMQ initialize
    // for(int i = n-2; i >= 0; i--) node[i] = max(node[i*2+1], node[i*2+2]);
  }
  void eval(int now, int l, int r) {
    if(lazy[now] != 0) {
      node[now] += lazy[now];
      if(r - l > 1) {
        lazy[2*now+1] += lazy[now] / 2;
        lazy[2*now+2] += lazy[now] / 2;
      }
      lazy[now] = 0;
    }
  }
  void evalMax(int now, int l, int r) {
    if(lazy[now] > 0) {
      node[now] = max(node[now], lazy[now]);
      if(r - l > 1) {
        lazy[2*now+1] = max(lazy[now], lazy[2*now+1]);
        lazy[2*now+2] = max(lazy[now], lazy[2*now+2]);
      }
      lazy[now] = 0;
    }
  }
  void add(int a, int b, ll x, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    eval(k, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[k] += (r-l) * x;
      eval(k, l, r);
    } else {
      add(a, b, x, 2*k+1, l, (l+r)/2);
      add(a, b, x, 2*k+2, (l+r)/2, r);
      node[k] = node[2*k+1] + node[2*k+2];
    }
  }
  ll getSum(int a, int b, int k=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return 0;
    eval(k, l, r);
    if(a <= l && r <= b) return node[k];
    ll res = 0;
    res += getSum(a, b, 2*k+1, l, (l+r)/2);
    res += getSum(a, b, 2*k+2, (l+r)/2, r);
    return res;
  }
  void updateMax(int a, int b, ll x, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    evalMax(now, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[now] = max(lazy[now], x);
      evalMax(now, l, r);
      return;
    }
    updateMax(a, b, x, 2*now+1, l, (l+r)/2);
    updateMax(a, b, x, 2*now+2, (l+r)/2, r);
    node[now] = max(node[2*now+1], node[2*now+2]);
  }
  ll getMax(int a, int b, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return 0;
    evalMax(now, l, r);
    if(a <= l && r <= b) return node[now];
    ll res = 0;
    res = max(res, getMax(a, b, 2*now+1, l, (l+r)/2));
    res = max(res, getMax(a, b, 2*now+2, (l+r)/2, r));
    return res;
  }
};

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  
  return 0;
}
