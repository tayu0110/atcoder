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
  ll n, k;
  cin >> n >> k;
  vector<ll> a(n);
  ll mx = 0;
  for(int i=0;i<n;i++) {
    cin >> a[i];
    mx = max(a[i], mx);
  }
  sort(a.begin(), a.end());
  ll l = 0, r = mx+1;
  while(r - l > 1) {
    ll nk = k;
    ll m = (r + l) / 2;
    for(int i=n-1;i>=0;i--){
      nk -= ceill((ld)a[i] / m) - 1;
      if(ceill((ld)a[i] / m) - 1 == 0) break;
    }
    if(nk < 0) l = m;
    else r = m;
  }
  for(int i=n-1;i>=0;i--) {
    k -= ceill((ld)a[i] / r) - 1;
  }
  if(k < 0) cout << l << endl;
  else cout << r << endl;
  return 0;
}
