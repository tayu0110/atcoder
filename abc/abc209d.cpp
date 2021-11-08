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
  int n, q;
  cin >> n >> q;
  Graph t(n);
  for(int i=0;i<n-1;i++) {
    int a, b;
    cin >> a >> b;
    a--;b--;
    t[a].push_back(b);
    t[b].push_back(a);
  }
  vector<int> dist(n, -1);
  queue<pii> nt;
  nt.push({0, 0});
  while(!nt.empty()) {
    auto p = nt.front();
    int now = p.first, nd = p.second;
    nt.pop();
    if(dist[now] >= 0) continue;
    dist[now] = nd;
    for(auto e : t[now]) {
      if(dist[e] >= 0) continue;
      nt.push({e, nd+1});
    }
  }
  while(q--) {
    int c, d;
    cin >> c >> d;
    c--;d--;
    int diff = abs(dist[c] - dist[d]);
    if(diff % 2 == 0) cout << "Town" << endl;
    else cout << "Road" << endl;
  }
  return 0;
}
