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
#define INF (1LL<<60)
#define inf (1<<29)
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, q;
  cin >> n >> q;
  vector<ll> x(n), r(n), h(n);
  for(int i=0;i<n;i++) {
    cin >> x[i] >> r[i] >> h[i];
  }
  while(q--) {
    ll a, b;
    cin >> a >> b;
    ld v = 0;
    for(int i=0;i<n;i++) {
      ll d = x[i], u = d + h[i];
      if(d >= b || u <= a) continue;
      if(d >= a && u <= b) {
        v += PI * r[i] * r[i] * h[i];
      } else if(d >= a && u > b) {
        ll nh = u - b;
        v += PI * r[i] * r[i] * (h[i] * h[i] * h[i] - nh * nh * nh) / h[i] / h[i];
      } else if(d < a && u <= b) {
        ll nh = u - a;
        v += PI * r[i] * r[i] * nh * nh * nh / h[i] / h[i];
      } else {
        ll nh = u - a;
        ll nnh = u - b;
        v += PI * r[i] * nh  * r[i] * nh * (nh * nh * nh - nnh * nnh * nnh) / nh / nh / h[i] / h[i];
      }
    }
    cout << v / 3 << endl;
  }
  return 0;
}
