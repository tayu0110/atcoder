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
int bs(ll a, vector<ll> &b) {
  int l = 0, r = b.size();
  while(r-l > 1) {
    int m = (r+l) / 2;
    if(b[m] > a) r = m;
    else l = m;
  }
  return r;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<ll> a(n), b(m);
  for(int i=0;i<n;i++) cin >> a[i];
  for(int i=0;i<m;i++) cin >> b[i];
  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  ll ans = INF;
  for(int i=0;i<n;i++) {
    int pos = bs(a[i], b);
    if(pos != m) ans = min(ans, abs(a[i]-b[pos]));
    pos--;
    if(pos >= 0) ans = min(ans, abs(a[i]-b[pos]));
  }
  cout << ans << endl;
  return 0;
}
