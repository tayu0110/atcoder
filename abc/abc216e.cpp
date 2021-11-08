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
  int n;
  ll k;
  cin >> n >> k;
  vector<ll> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  map<ll, int> mp2;
  sort(a.begin(), a.end(), greater<ll>());
  int cnt = 0;
  vector<pll> mp;
  for(int i=0;i<n;i++) {
    if(mp2.find(a[i]) == mp2.end()) {
      mp2[a[i]] = cnt;
      mp.push_back({a[i], 0});
      cnt++;
    }
  }
  for(int i=0;i<n;i++) {
    mp[mp2[a[i]]].second++;
  }
  ll ans = 0;
  for(int i=0;i<cnt;i++) {
    if(i != cnt-1) {
      ll s = mp[i].first, t = mp[i+1].first;
      if((s - t) * mp[i].second <= k) {
        ans += ((s+1) * s / 2 - (t+1) * t / 2) * mp[i].second;
        k -= (s-t) * mp[i].second;
      } else {
        ll d = k / mp[i].second;
        ans += ((s+1) * s / 2 - (s-d+1) * (s-d) / 2) * mp[i].second;
        s -= d;
        ans += s * (k % mp[i].second);
        break;
      }
      mp[i+1].second += mp[i].second;
    } else {
      ll s = mp[i].first;
      if(s * mp[i].second <= k) {
        ans += (s+1) * s / 2 * mp[i].second;
        break;
      } else {
        ll d = k / mp[i].second;
        ll t = max(0LL, s-d);
        ans += ((s+1) * s / 2 - (t+1) * t / 2) * mp[i].second;
        s -= d;
        ans += s * (k % mp[i].second);
      }
    }
  }
  cout << ans << endl;
  return 0;
}
