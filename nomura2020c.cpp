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
  vector<ll> a(n+1);
  for(int i=0;i<n+1;i++) {
    cin >> a[i];
  }
  if(n == 0) {
    if(a[0] == 1) cout << 1 << endl;
    else cout << -1 << endl;
    return 0;
  }
  if(a[0] != 0) {
    cout << -1 << endl;
    return 0;
  }
  ll ans = 1, now = 1;
  ll sum = 0;
  for(int i=0;i<n+1;i++) sum += a[i];
  for(int i=1;i<n+1;i++) {
    now = 2*now - a[i];
    if(now < 0) {
      cout << -1 << endl;
      return 0;
    }
    sum -= a[i];
    now = min(now, sum);
    ans += now + a[i];
  }
  cout << ans << endl;
  return 0;
}
