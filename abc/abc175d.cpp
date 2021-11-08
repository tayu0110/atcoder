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
  ll k;
  cin >> n >> k;
  vector<int> p(n);
  vector<ll> c(n);
  for(int i=0;i<n;i++) cin >> p[i], p[i]--;
  for(int i=0;i<n;i++) cin >> c[i];
  ll ans = -INF;
  for(int i=0;i<n;i++) {
    ll sum = 0;
    ll cnt = 0;
    int now = i;
    do {
      now = p[now];
      sum += c[now];
      ans = max(ans, sum);
      cnt++;
      if(cnt == k) break;
    } while(now != i);
    if(sum < 0) continue;
    if(cnt == k) continue;
    sum *= k / cnt - 1;
    ans = max(ans, sum);
    for(int j=0;j<k%cnt+cnt;j++) {
      now = p[now];
      sum += c[now];
      ans = max(ans, sum);
    }
  }
  cout << ans << endl;
  return 0;
}
