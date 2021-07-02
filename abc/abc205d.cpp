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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<ll> a;
  set<ll> ck;
  for(int i=0;i<n;i++) {
    ll t;
    cin >> t;
    if(ck.find(t) == ck.end()) a.push_back(t);
    ck.insert(t);
  }
  n = a.size();
  sort(a.begin(), a.end());
  vector<ll> b(n);
  for(int i=0;i<n;i++) {
    if(i == 0) b[i] = a[i]-1;
    else {
      b[i] = a[i] - a[i-1] - 1;
      b[i] += b[i-1];
    }
  }
  // for(auto e : b) cout << e << " ";
  // cout << endl;
  vector<ll> res;
  while(q--) {
    ll k;
    cin >> k;
    auto it = lower_bound(b.begin(), b.end(), k);
    if(it == b.end()) {
      res.push_back(k + n);
      continue;
    }
    ll pos = it - b.begin();
    res.push_back(a[pos] - (b[pos]-k+1));
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
