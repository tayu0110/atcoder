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

#define BIL ((ll)1e9)
#define MOD ((ll)1e9+7)
#define INF (1LL<<60)           //1LL<<63でオーバーフロー
#define inf (1<<29)             //1<<29でオーバーフロー

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    int n;
    cin >> n;
    vector<ll> d(0), m(0);
    for(int i=0;i<2*n;i++) {
      ll x, y;
      cin >> x >> y;
      x = abs(x);
      y = abs(y);
      if(x == 0) m.emplace_back(y);
      else d.emplace_back(x);
    }
    sort(d.begin(), d.end());
    sort(m.begin(), m.end());
    ld ans = 0;
    for(int i=0;i<n;i++) {
      ld a = d[i];
      ld b = m[i];
      ans += sqrtl(a*a + b*b);
    }
    cout << ans << endl;
  }
  return 0;
}
