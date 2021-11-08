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
int main(int argc, char** argv) {
  int h, w;
  ll c;
  cin >> h >> w >> c;
  vector<vector<ll>> a(h, vector<ll>(w));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> a[i][j];
  vector<vector<int>> d(h, vector<int>(w)), rd(h, vector<int>(w));
  vector<vector<ll>> dc(h, vector<ll>(w)), rdc(h, vector<ll>(w));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) d[i][j] = i+j, dc[i][j] = a[i][j]-c*d[i][j];
  for(int i=0;i<h;i++) for(int j=w-1;j>=0;j--) rd[i][j] = i+w-1-j, rdc[i][j] = a[i][j]-c*rd[i][j];
  for(int i=0;i<h;i++) for(int j=1;j<w;j++) dc[i][j] = min(dc[i][j], dc[i][j-1]);
  for(int i=1;i<h;i++) for(int j=0;j<w;j++) dc[i][j] = min(dc[i][j], dc[i-1][j]);
  for(int i=0;i<h;i++) for(int j=w-2;j>=0;j--) rdc[i][j] = min(rdc[i][j], rdc[i][j+1]);
  for(int i=1;i<h;i++) for(int j=w-1;j>=0;j--) rdc[i][j] = min(rdc[i][j], rdc[i-1][j]);
  ll ans = INF;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    ll k = a[i][j] + c*d[i][j];
    ll s = INF;
    if(i-1 >= 0 && j-1 >= 0) s = min(s, dc[i-1][j-1]);
    if(i-1 >= 0) s = min(s, dc[i-1][j]);
    if(j-1 >= 0) s = min(s, dc[i][j-1]);
    ans = min(ans, k + s);
    ll l = a[i][j] + c*rd[i][j];
    ll t = INF;
    if(i-1 >= 0 && j+1 < w) t = min(t, rdc[i-1][j+1]);
    if(i-1 >= 0) t = min(t, rdc[i-1][j]);
    if(j+1 < w) t = min(t, rdc[i][j+1]);
    ans = min(ans, l + t);
  }
  cout << ans << endl;
}