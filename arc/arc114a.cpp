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
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> x(n);
  set<int> ck;
  for(int i=0;i<n;i++) {
    cin >> x[i];
    ck.insert(x[i]);
  }
  vector<ll> p = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47};
  int ps = p.size();
  ll ans = INF;
  for(int i=1;i<(1<<ps);i++) {
    ll now = 1;
    for(int j=0;j<ps;j++) {
      if(i & (1 << j)) now *= p[j];
    }
    bool flag = true;
    for(int j=0;j<n;j++) {
      ll g = gcd(now, x[j]);
      if(g == 1) {
        flag = false;
        break;
      }
    }
    if(!flag) continue;
    ans = min(ans, now);
  }
  cout << ans << endl;
  return 0;
}
