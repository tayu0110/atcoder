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
const ld eps = 1e-9;
ld dist(vector<pair<ld, ld>> &p, ld x, ld y) {
  ld mx = 0;
  for(auto e : p) {
    ld dx = e.first, dy = e.second;
    ld s = abs(dx-x), t = abs(dy-y);
    mx = max(mx, s*s+t*t);
  }
  return sqrtl(mx);
}
ld yt(vector<pair<ld, ld>> &p, ld x) {
  ld l = 0, r = 1500;
  int t = 100;
  while(t--) {
    ld s = (l*2+r) / 3;
    ld t = (l+2*r) / 3;
    ld ds = dist(p, x, s);
    ld dt = dist(p, x, t);
    if(ds < dt-eps) r = t;
    else l = s;
  }
  if(dist(p, x, l) < dist(p, x, r)-eps) return l;
  else return r;
}
ld xt(vector<pair<ld, ld>> &p) {
  ld l = 0, r = 1500;
  int t = 100;
  while(t--) {
    ld s = (l*2+r) / 3;
    ld t = (l+2*r) / 3;
    ld ds = dist(p, s, yt(p, s));
    ld dt = dist(p, t, yt(p, t));
    if(ds < dt-eps) r = t;
    else l = s;
  }
  ld dl = dist(p, l, yt(p, l));
  ld dr = dist(p, r, yt(p, r));
  if(dl < dr-eps) return dl;
  else return dr;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pair<ld, ld>> p(n);
  for(int i=0;i<n;i++) cin >> p[i].first >> p[i].second;
  cout << xt(p) << endl;
  return 0;
}
