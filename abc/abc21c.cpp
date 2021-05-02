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
using heap = priority_queue<pair<pii, ll>, vector<pair<pii, ll>>, greater<pair<pii, ll>>>;

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
  int a, b;
  cin >> a >> b;
  a--;b--;
  int m;
  cin >> m;
  Graph g(n);
  for(int i=0;i<m;i++) {
    int x, y;
    cin >> x >> y;
    x--;y--;
    g[x].push_back(y);
    g[y].push_back(x);
  }
  vector<int> d(n, inf);
  d[a] = 0;
  vector<ll> c(n, 0);
  c[a] = 1;
  queue<int> nt;
  nt.push(a);
  vector<bool> ck2(n, false);
  while(!nt.empty()) {
    int now = nt.front();
    nt.pop();
    if(ck2[now]) continue;
    ck2[now] = true;
    for(int i=0;i<g[now].size();i++) {
      int j = g[now][i];
      if(ck2[j]) continue;
      d[j] = min(d[j], d[now]+1);
      nt.push(j);
    }
  }
  queue<int> nt2;
  nt2.push(a);
  set<int> ck;
  while(!nt2.empty()) {
    int now = nt2.front();
    nt2.pop();
    if(ck.find(now) != ck.end()) continue;
    for(int i=0;i<g[now].size();i++) {
      int j = g[now][i];
      if(d[now] - d[j] == 1) {
        c[now] += c[j];
        c[now] %= MOD;
      } else {
        if(ck.find(j) == ck.end()) nt2.push(j);
      }
    }
    ck.insert(now);
  }
  cout << c[b] % MOD << endl;
  return 0;
}
