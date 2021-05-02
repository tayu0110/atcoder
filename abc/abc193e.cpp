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
  Edge(int to, long long weight) : to(to), weight(weight) {}
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

inline ll mod(ll a, ll m) {
  return (a%m + m) % m;
}

ll extGCD(ll a, ll b, ll &p, ll &q) {
  if(b == 0) {
    p = 1;
    q = 0;
    return a;
  }
  ll d = extGCD(b, a%b, q, p);
  q -= a/b * p;
  return d;
}

pll crt(ll b1, ll m1, ll b2, ll m2) {
  ll p, q;
  ll d = extGCD(m1, m2, p, q);
  if((b2 - b1) % d != 0) return make_pair(0, -1);
  ll m = m1 * (m2/d);
  ll tmp = (b2 - b1) / d * p % (m2/d);
  ll r = mod(b1 + m1 * tmp, m);
  return make_pair(r, m);
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=0;i<t;i++){
    ll x, y, p, q;
    cin >> x >> y >> p >> q;
    ll m1 = 2*(x+y);
    ll m2 = p+q;
    ll ans = INF;
    for(ll i = x; i < x+y; i++) {
      for(ll j = p; j < p+q; j++) {
        pll k = crt(i, m1, j, m2);
        if(k.second == -1) continue;
        ans = min(ans, k.first);
      }
    }
    if(ans == INF) cout << "infinity" << endl;
    else cout << ans << endl;
  }
  return 0;
}
