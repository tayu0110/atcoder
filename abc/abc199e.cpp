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
  int n, m;
  cin >> n >> m;
  map<int, vector<pii>> mp;
  for(int i=0;i<m;i++) {
    int x, y, z;
    cin >> x >> y >> z;
    mp[x].push_back({y, z});
  }
  vector<ll> dp(1<<n);
  dp[0] = 1;
  for(int i=0;i<(1<<n);i++) {
    vector<int> q(n+1);
    for(int j=1;j<n+1;j++) {
      if(i & (1<<(j-1))) q[j] = q[j-1]+1;
      else q[j] = q[j-1];
    }
    int k = q[n];    
    auto itr = mp.find(k+1);
    for(int j=0;j<n;j++) {
      if(i & (1<<j)) continue;
      bool f = true;
      if(itr != mp.end()) {
        vector<pii> &r = itr->second;
        for(int l=0;l<r.size();l++) {
          int y = r[l].first;
          int z = r[l].second;
          if(j < y) z--;
          if(q[y] > z) {
            f = false;
            break;
          }
        }
      }
      if(!f) continue;
      int t = i | (1<<j);
      dp[t] += dp[i];
    }
  }
  cout << dp[(1<<n)-1] << endl;
  return 0;
}
