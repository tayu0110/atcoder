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
  vector<ll> s(n);
  bool zf = false;
  for(int i=0;i<n;i++) {
    cin >> s[i];
    if(s[i] == 0) zf = true;
  }
  if(zf) {
    cout << n << endl;
    return 0;
  }
  if(k == 0) {
    cout << 0 << endl;
    return 0;
  }
  ll t = 1;
  int f = 0;
  int ans = 0;
  for(int i=0;i<n;i++) {
    t *= s[i];
    if(t <= k) {
      ans = max(ans, i-f+1);
      continue;
    }
    while(t > k && f < i) {
      t /= s[f];
      f++;
    }
    if(t <= k) ans = max(ans, i-f+1);
  }
  cout << ans << endl;
  return 0;
}
