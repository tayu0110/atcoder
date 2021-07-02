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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int w, n;
  cin >> w >> n;
  LazySegmentTree lst(w);
  for(int i=0;i<n;i++) {
    int l, r;
    cin >> l >> r;
    l--;
    int mx = lst.getMax(l, r);
    cout << mx+1 << endl;
    lst.updateMax(l, r, mx+1);
  }
  return 0;
}
