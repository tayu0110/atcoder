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
  int n, m;
  cin >> n >> m;
  vector<ll> h(n);
  vector<ll> w(m);
  for(int i=0;i<n;i++) {
    cin >> h[i];
  }
  for(int i=0;i<m;i++) {
    cin >> w[i];
  }
  sort(h.begin(), h.end());
  sort(w.begin(), w.end());
  vector<ll> p(n), b(n);
  for(int i=0;i<n;i++) {
    if(i%2==1) {
      p[i] = h[i] - h[i-1];
      if(i != 1) p[i] += p[i-2];
      p[i-1] = p[i];
    } else {
      if(i != 0) {
        b[i] = h[i] - h[i-1] + b[i-2];
        b[i-1] = b[i];
      }
    }
  }
  ll ans = INF;
  for(int i=0;i<m;i++) {
    ll mx = 0;
    auto it = upper_bound(h.begin(), h.end(), w[i]);
    int j = it - h.begin();
    if(j % 2 == 0) {
      mx = h[j] - w[i];
      if(j != 0) mx += p[j-1];
      if(j != n-1) mx += b[n-2] - b[j];
    } else {
      mx = w[i] - h[j-1];
      if(j != 1) mx += p[j-2];
      if(j != n) mx += b[n-1] - b[j-1];
    }
    ans = min(ans, mx);
  }
  cout << ans << endl;
  return 0;
}
