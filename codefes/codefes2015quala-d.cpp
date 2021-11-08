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
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll n;
  int m;
  cin >> n >> m;
  vector<ll> x(m);
  for(int i=0;i<m;i++) cin >> x[i];
  ll l = -1, r = 2001001001;
  while(r-l > 1) {
    ll mid = (r+l) / 2;
    ll end = 0;
    for(int i=0;i<m;i++) {
      ll now = x[i];
      ll k = mid;
      if(end+1 < now) {
        ll s = now - (end+1);
        if(s > k) break;
        ll t = k - s;
        if(t/2 > t-s) {
          k = t/2;
        } else {
          k = t-s;
        }
      }
      end = max(end, max(now + k, x[i]));
    }
    if(end < n) l = mid;
    else r = mid;
  }
  cout << r << endl;
  return 0;
}
