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
  int n;
  cin >> n;
  vector<int> a(n), b(n);
  for(int i=0;i<n;i++) {
    cin >> a[i];
    b[i] = a[i];
  }
  reverse(b.begin(), b.end());
  vector<int> dp(n, inf), rdp(n, inf);
  vector<int> p(n), q(n);
  for(int i=0;i<n;i++) {
    auto it = lower_bound(dp.begin(), dp.end(), a[i]);
    auto rit = lower_bound(rdp.begin(), rdp.end(), b[i]);
    if(it != a.end()) *it = a[i];
    if(rit != b.end()) *rit = b[i];
    p[i] = it - dp.begin() + 1;
    q[i] = rit - rdp.begin() + 1;
  }
  int ans = 0;
  for(int i=0;i<n;i++) ans = max(ans, p[i]+q[n-1-i]-1);
  cout << ans << endl;
  return 0;
}
