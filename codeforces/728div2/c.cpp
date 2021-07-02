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
  int t;
  cin >> t;
  vector<ll> ans;
  while(t--) {
    int n;
    cin >> n;
    vector<ll> d(n);
    for(int i=0;i<n;i++) {
      cin >> d[i];
    }
    if(n < 3) {
      ans.push_back(0);
      continue;
    }
    sort(d.begin(), d.end());
    vector<ll> nd(n-1);
    for(int i=1;i<n;i++) {
      nd[i-1] = d[i] - d[i-1];
    }
    n--;
    vector<ll> t(n);
    ll s = n;
    for(int i=0;i<(n+1)/2;i++) {
      if(i == 0) t[i] = s;
      else t[i] = t[i-1] + s;
      s-=2;
      // cout << "i: " << i << " t[i]: " << t[i] << endl;
    }
    for(int i=n-1;i>=(n+1)/2;i--) {
      t[i] = t[n-1-i];
      // cout << "i: " << i << " t[i]: " << t[i] << endl;
    }
    ll res = 0;
    for(int i=0;i<n;i++) res -= nd[i] * t[i];
    for(int i=0;i<n;i++) res += nd[i];
    ans.push_back(res);
  }
  for(auto e : ans) cout << e << endl;
  return 0;
}
