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
using heap = priority_queue<pll, vector<pll>>;

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  const int m = 200005;
  int n, q;
  cin >> n >> q;
  vector<ll> a(n);
  vector<int> b(n);
  vector<multiset<int>> s(m);
  multiset<int> t;
  for(int i=0;i<n;i++) {
    cin >> a[i] >> b[i];
    s[b[i]].insert(a[i]);
  }
  for(int i=0;i<m;i++) {
    if(s[i].empty()) continue;
    auto mx = s[i].rbegin();
    t.insert(*mx);
  }
  while(q--) {
    int c, d;
    cin >> c >> d;
    c--;
    auto it = s[b[c]].find(a[c]);
    s[b[c]].erase(it);
    if(!s[b[c]].empty()) {
      auto mx = s[b[c]].rbegin();
      if(a[c] > *mx) {
        auto tit = t.find(a[c]);
        t.erase(tit);
        t.insert(*mx);
      }
    } else {
      auto tit = t.find(a[c]);
      t.erase(tit);
    }
    if(!s[d].empty()) {
      auto mx = s[d].rbegin();
      if(a[c] > *mx) {
        auto tit = t.find(*mx);
        t.erase(tit);
        t.insert(a[c]);
      }
    } else {
      t.insert(a[c]);
    }
    s[d].insert(a[c]);
    b[c] = d;
    auto mn = t.begin();
    cout << *mn << endl;
  }
  return 0;
}
