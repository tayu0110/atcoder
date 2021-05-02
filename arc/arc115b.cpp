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

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<ll>> c(n, vector<ll>(n, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      cin >> c[i][j];
    }
  }
  ll v = INF;
  for(int i=0;i<n;i++) {
    v = min(v, c[i][0]);
  }
  vector<ll> a(n);
  for(int i=0;i<n;i++) {
    a[i] = c[i][0] - v;
  }
  vector<ll> b(n);
  for(int i=0;i<n;i++) {
    ll k = 0;
    for(int j=0;j<n;j++) {
      if(j == 0) {
        k = c[j][i] - a[j];
        continue;
      }
      ll m = c[j][i] - a[j];
      if(k != m) {
        cout << "No" << endl;
        return 0;
      }
    }
    if(k < 0) {
      cout << "No" << endl;
      return 0;
    }
    b[i] = k;
  }
  cout << "Yes" << endl;
  for(int i=0;i<n;i++) {
    cout << a[i];
    if(i != n-1) cout << " ";
  }
  cout << endl;
  for(int i=0;i<n;i++) {
    cout << b[i];
    if(i != n-1) cout << " ";
  }
  cout << endl;
  return 0;
}
