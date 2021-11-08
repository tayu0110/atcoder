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
struct LazySegmentTree {
 private:
  int n;
  vector<ll> node, lazy;
 public:
  void init(int sz) {
    n = 1;
    while(n < sz) n *= 2;
    node.assign(2*n-1, INF); lazy.assign(2*n-1, INF);
  }
  LazySegmentTree(int sz) {
   init(sz);
  }
  void evalMin(int now, int l, int r) {
    if(lazy[now] < INF) {
      node[now] = min(node[now], lazy[now]);
      if(r - l > 1) {
        lazy[2*now+1] = min(lazy[now], lazy[2*now+1]);
        lazy[2*now+2] = min(lazy[now], lazy[2*now+2]);
      }
      lazy[now] = INF;
    }
  }
  void updateMin(int a, int b, ll x, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    evalMin(now, l, r);
    if(b <= l || r <= a) return;
    if(a <= l && r <= b) {
      lazy[now] = min(lazy[now], x);
      evalMin(now, l, r);
      return;
    }
    updateMin(a, b, x, 2*now+1, l, (l+r)/2);
    updateMin(a, b, x, 2*now+2, (l+r)/2, r);
    node[now] = min(node[2*now+1], node[2*now+2]);
  }
  ll getMin(int a, int b, int now=0, int l=0, int r=-1) {
    if(r < 0) r = n;
    if(b <= l || r <= a) return INF;
    evalMin(now, l, r);
    if(a <= l && r <= b) return node[now];
    ll res = INF;
    res = min(res, getMin(a, b, 2*now+1, l, (l+r)/2));
    res = min(res, getMin(a, b, 2*now+2, (l+r)/2, r));
    return res;
  }
};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  LazySegmentTree lst(n);
  lst.updateMin(0, 1, 0);
  vector<pair<pii, ll>> p(m);
  for(int i=0;i<m;i++) {
    int l, r;
    ll c;
    cin >> l >> r >> c;
    l--;
    p[i] = {{l, r}, c};
  }
  sort(p.begin(), p.end());
  for(int i=0;i<m;i++) {
    int l = p[i].first.first, r = p[i].first.second;
    ll c = p[i].second;
    ll mn = lst.getMin(l, r);
    if(l == 0) l++;
    lst.updateMin(l, r, mn+c);
  }
  ll ans = lst.getMin(n-1, n);
  if(ans == INF) ans = -1;
  cout << ans << endl;
  return 0;
}
