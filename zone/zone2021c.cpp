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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<ll>> p(n, vector<ll>(5, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<5;j++) cin >> p[i][j];
  }
  ll l = -1, r = 1001001001;
  while(r-l > 1) {
    ll m = (r+l) / 2;
    map<int, int> mp;
    for(int i=0;i<n;i++) {
      int t = 0;
      for(int j=0;j<5;j++) {
        if(p[i][j] >= m) t |= 1 << j;
      }
      mp[t]++;
    }
    auto check = [&](int t) {
      return mp.find(t) == mp.end() || mp[t] == 0;
    };
    bool f = false;
    for(int i=0;i<(1<<5);i++) {
      if(check(i)) continue;
      mp[i]--;
      for(int j=0;j<(1<<5);j++) {
        if(check(j)) continue;
        mp[j]--;
        for(int k=0;k<(1<<5);k++) {
          if(check(k)) continue;
          int t = i | j | k;
          if(t == (1<<5)-1) f = true;
        }
        mp[j]++;
      }
      mp[i]++;
    }
    if(f) l = m;
    else r = m;
  }
  cout << l << endl;
  return 0;
}
