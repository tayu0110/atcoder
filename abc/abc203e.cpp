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
using heap = priority_queue<int, vector<int>, greater<int>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

ll n, m;
set<ll> ans;
void solve(vector<pll>& p) {
  map<ll, set<ll>> q;
  for(int i=0;i<p.size();i++) {
    ll x = p[i].first;
    ll y = p[i].second;
    q[y].insert(x);
  }
  queue<pll> nt;
  nt.push({0, n});
  set<pll> ck;
  while(!nt.empty()) {
    pll now = nt.front();
    nt.pop();
    if(ck.find(now) != ck.end()) continue;
    ck.insert(now);
    ll x = now.first;
    ll y = now.second;
    if(q.find(y+1) != q.end()) {
      auto it = q[y+1].upper_bound(x);
      while(it != q[y+1].end()) nt.push({*it++, y+1});
    }
    if(q.find(y-1) != q.end()) {
      auto it = q[y-1].upper_bound(x);
      while(it != q[y-1].end()) nt.push({*it++, y-1});
    }
    if(q[y].upper_bound(x) == q[y].end()) {
      ans.insert(y);
    }
  }
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n >> m;
  vector<pll> p(m);
  for(int i=0;i<m;i++) {
    ll x, y;
    cin >> x >> y;
    p[i] = {x, y};
  }
  sort(p.begin(), p.end());
  solve(p);
  cout << ans.size() << endl;
  return 0;
}
