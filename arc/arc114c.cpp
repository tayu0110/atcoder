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

ll combination(ll n, ll r){
  if(r == 0) return 1;
  if(n-r < r) r = n-r;
  ll k = r;
  ll res = 1;
  ll d = 1;
  for(int i=0;i<k;i++) {
    res *= n;
    n--;
    while(res % d == 0 && d <= r) {
      res /= d;
      d++;
    }
  }
  return res;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, m;
  cin >> n >> m;
  vector<vector<ll>> d(n, vector<ll>(m+2, 0));
  for(int i=1;i<m+1;i++) {
    d[0][i] = 1;
  }
  for(int i=0;i<n-1;i++) {
    for(int j=1;j<m+1;j++) {
      d[i+1][j-1] += d[i][j];
      d[i+1][j+1] += d[i][j];
      d[i][j] += d[i+1][j-1];
      if(i-1 >=0) {
        d[i-1][j-1] += d[i][j];
        d[i-1][j+1] += d[i][j];
      }
    }
  }
  ll ans = 0;
  for(int i=0;i<n;i++) {
    for(int j=1;j<m+1;j++) {
      ans += d[i][j];
      ans %= 998244353;
    }
  }
  cout << ans << endl;
  return 0;
}
