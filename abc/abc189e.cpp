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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<pll> p(n);
  for(int i=0;i<n;i++) {
    cin >> p[i].first >> p[i].second;
  }
  struct A {
    ll px, py, qx, qy;
    bool f;
    A() : px(0), py(0), qx(1), qy(1), f(true) {}
    void MovX(ll x) {
      px = 2*x - px;
      qx *= -1;
    }
    void MovY(ll y) {
      py = 2*y - py;
      qy *= -1;
    }
    void Rot() {
      ll p = -px, q = -qx;
      px = py, qx = qy;
      py = p, qy = q;
      f = !f;
    }
    void rRot() {
      ll p = -py, q = -qy;
      py = px, qy = qx;
      px = p, qx = q;
      f = !f;
    }
    pll Pt(ll x, ll y) {
      if(f) return { px + x*qx, py + y*qy};
      else return {px + y*qx, py + x*qy};
    }
  };
  A trans;
  int m;
  cin >> m;
  vector<A> t(m);
  for(int i=0;i<m;i++) {
    int k;
    cin >> k;
    if(k == 1) trans.Rot();
    else if(k == 2) trans.rRot();
    else if(k == 3) {
      ll p;
      cin >> p;
      trans.MovX(p);
    } else if(k == 4) {
      ll p;
      cin >> p;
      trans.MovY(p);
    }
    t[i] = trans;
  }
  int q;
  cin >> q;
  vector<pll> res;
  while(q--) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    if(a < 0) {
      res.push_back(p[b]);
      // cout << p[b].first << " " << p[b].second << endl;
      continue;
    }
    pll pt = t[a].Pt(p[b].first, p[b].second);
    res.push_back(pt);
    // cout << pt.first << " " << pt.second << endl;
  }
  for(auto e : res) {
    cout << e.first << " " << e.second << endl;
  }
  return 0;
}
