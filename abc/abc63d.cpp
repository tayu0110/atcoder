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

#define DEBUG(var) cout << #var << ": " << var << "\t";
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
  ll a, b;
  cin >> n >> a >> b;
  vector<ll> h(n);
  for(int i=0;i<n;i++) cin >> h[i];
  if(n == 1) {
    cout << (h[0]-1) / a + 1 << endl;
    return 0;
  }
  sort(h.begin(), h.end(), greater<ll>());
  ll k = a - b;
  ll l = 0, r = 100100100100100;
  while(r-l > 1) {
    ll m = (r+l) / 2;
    ll s = 0, t = h[0];
    while(t-s > 1) {
      ll v = (s+t) / 2;
      ll cnt = m;
      for(int i=0;i<n;i++) {
        ll d = h[i] - v;
        if(d <= 0) break;
        cnt -= (d-1) / k + 1;
      }
      if(cnt < 0) s = v;
      else t = v;
    }
    // DEBUG(t);DEBUG(b);DEBUG(m);DEBUG_EN(b*m);
    if((t-1) / b + 1 <= m) r = m;
    else l = m;
    // DEBUG(l);DEBUG_EN(r);
  }
  cout << r << endl;
  return 0;
}
