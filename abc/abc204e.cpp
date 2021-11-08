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
  long long c, d;
  Edge() : to(0), c(0), d(0) {}
  Edge(int to, long long c, long long d) : to(to), c(c), d(d) {}
  Edge(const Edge& e) : to(e.to), c(e.c), d(e.d) {}
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
ll td(ll t, ll c, ll d) {
  ld l = 0, r = d;
  while(r-l > 0.0000001) {
    ld nl = (2*l+r) / 3;
    ld nr = (l+2*r) / 3;
    ld p = nl + c + (ld)d / (t+1+nl);
    ld q = nr + c + (ld)d / (t+1+nr);
    if(p-q < 0.00000001) r = nr;
    else l = nl;
  }
  ld p = l + c + (ld)d / (t+1+l);
  ld q = r + c + (ld)d / (t+1+r);
  if(q-p < 0.00000001) l = r;
  ll u = floor(l) + c + d / (t+1+floor(l));
  ll v = ceil(l) + c + d / (t+1+ceil(l));
  if(u < v) return floor(l);
  else return ceil(l);
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    ll c, d;
    cin >> a >> b >> c >> d;
    a--;b--;
    t[a].push_back(Edge(b, c, d));
    t[b].push_back(Edge(a, c, d));
  }
  vector<ll> d(n, -1);
  heap<pll> nt;
  nt.push({0, 0});
  while(!nt.empty()) {
    auto p = nt.top();
    nt.pop();
    int now = p.second;
    ll nd = p.first;
    if(d[now] >= 0) continue;
    d[now] = nd;
    for(auto e : t[now]) {
      if(d[e.to] >= 0) continue;
      ll s = td(nd, e.c, e.d);
      nt.push({nd+s+e.c+(e.d/(nd+1+s)), e.to});
    }
  }
  cout << d[n-1] << endl;
  return 0;
}
