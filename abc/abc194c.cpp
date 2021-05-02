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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) {
    cin >> a[i];
  }
  vector<ll> b(n);
  vector<ll> c(n);
  for(int i=0;i<n;i++) {
    b[i] = a[i] * a[i];
    if(i==0) c[i] = a[i];
    else c[i] = c[i-1] + a[i];
  }
  vector<ll> d(n);
  for(int i=0;i<n;i++) {
    if(i==0) d[i] = b[i];
    else d[i] = d[i-1] + b[i];
  }
  ll ans = 0;
  for(int i=1;i<n;i++) {
    ans += (i)*b[i] - 2 * a[i] * c[i-1] + d[i-1];
  }
  cout << ans << endl;
  return 0;
}
