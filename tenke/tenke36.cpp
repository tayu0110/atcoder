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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<pll> p(4);
  vector<pll> xy(n);
  for(int i=0;i<n;i++) {
    ll x, y;
    cin >> x >> y;
    xy[i] = {x, y};
    int t = 0;
    if(x >= 0 && y < 0) t = 1;
    else if(x < 0 && y >= 0) t = 2;
    else if(x < 0 && y < 0) t = 3;
    if(abs(p[t].first)+abs(p[t].second) < abs(x)+abs(y)) p[t] = {x, y};
  }
  while(q--) {
    int t;
    cin >> t;
    t--;
    ll x = xy[t].first, y = xy[t].second;
    ll res = 0;
    for(int i=0;i<4;i++) {
      ll px = p[i].first, py = p[i].second;
      res = max(res, abs(x-px)+abs(y-py));
    }
    cout << res << endl;
  }
  return 0;
}
