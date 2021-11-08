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
ll comb(ll n, ll k) {
  if(k == 0) return 1;
  ll res = 1;
  ll d = 1;
  for(ll i=n;i>n-k;i--) {
    res *= i;
    while(d <= k && (res % d == 0)) res /= d, d++;
  }
  while(d <= k && (res % d) == 0) res /= d, d++;
  return res;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, a, b;
  cin >> n >> a >> b;
  vector<ll> v(n);
  for(int i=0;i<n;i++) cin >> v[i];
  sort(v.begin(), v.end(), greater<ll>());
  ll mx = 0;
  int cnt = 1;
  ll sum = 0;
  ll ans = 0;
  for(ll i=0;i<b;i++) {
    sum += v[i];
    if(i < a-1) continue;
    if(mx * (i+1) > sum * cnt) continue;
    if(mx * (i+1) < sum * cnt) {
      mx = sum;
      cnt = i+1;
      ans = 0;
    }
    ll l = i, r = i;
    while(l >= 0 && v[l] == v[i]) l--;
    while(r < n && v[r] == v[i]) r++;
    l++;
    ans += comb(r-l, i-l+1);
  }
  cout << (ld)mx / cnt << endl;
  cout << ans << endl;
  return 0;
}
