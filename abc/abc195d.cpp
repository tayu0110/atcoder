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
  // cin.tie(0);
  // ios::sync_with_stdio(0);
  // cout << fixed << setprecision(20);
  int n, m, q;
  cin >> n >> m >> q;
  vector<int> w(n), v(n);
  for(int i=0;i<n;i++) {
    int k, r;
    cin >> k >> r;
    w[i] = k;
    v[i] = r;
  }
  vector<pair<int, int>> x(m);
  for(int i=0;i<m;i++) {
    cin >> x[i].first;
    x[i].second = i;
  }
  sort(x.begin(), x.end());
  for(int k=0;k<q;k++) {
    ll l, r;
    cin >> l >> r;
    l--;
    r--;
    set<int> ck;
    for(int i=0;i<m;i++) {
      if(x[i].second >= l && x[i].second <= r) continue;
      int nowj = -1;
      int xi = x[i].first;
      for(int j=0;j<n;j++) {
        if(w[j] > xi) continue;
        if(ck.find(j) != ck.end()) continue;
        if(nowj == -1) {
          nowj = j;
          continue;
        }
        if(v[nowj] < v[j]) nowj = j;
      }
      if(nowj != -1) ck.insert(nowj);
    }
    ll ans = 0;
    for(auto e : ck) ans += v[e];
    cout << ans << endl;
  }
  return 0;
}
