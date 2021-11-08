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
struct BIT {
  ll n;
  vector<ll> a;
  BIT(ll n) : n(n), a(n+1,0) {}
  void add(ll i, ll x) {
    i++;
    if(i == 0) return;
    for(ll k = i; k <= n; k += (k & -k)) a[k] += x;
    return;
  }
  // The sum of [i, j)
  ll sum(ll i, ll j) {
    return sum_sub(j-1) - sum_sub(i-1);
  }
  ll sum_sub(ll i) {
    i++;
    ll s = 0;
    if(i == 0) return s;
    for(ll k = i; k > 0; k -= (k & -k)) s += a[k];
    return s;
  }
  ll lower_bound(ll x) {
    if(x <= 0) return 0;
    ll i = 0, r = 1;
    while(r < n) r = r << 1;
    for(int len = r; len >= 0; len = len >> 1) {
      if(i+len < n && a[i+len] < x) {
        x -= a[i+len];
        i += len;
      }
    }
    return i;
  }
};
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<int> c(n);
  for(int i=0;i<n;i++) cin >> c[i], c[i]--;
  vector<pair<pii, int>> p(q);
  for(int i=0;i<q;i++) {
    cin >> p[i].first.second >> p[i].first.first;
    p[i].second = i;
  }
  sort(p.begin(), p.end());
  vector<int> good(n, -1);
  BIT bit(n);
  int j = 0;
  vector<int> res(q);
  for(int i=0;i<q;i++) {
    int l = p[i].first.second, r = p[i].first.first;
    int idx = p[i].second;
    for(;j<r;j++) {
      if(good[c[j]] >= 0) bit.add(good[c[j]], -1);
      good[c[j]] = j;
      bit.add(j, 1);
    }
    res[idx] = bit.sum(l-1, r);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
