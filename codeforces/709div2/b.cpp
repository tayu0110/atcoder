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
  int t;
  cin >> t;
  while(t--) {
    int n;
    cin >> n;
    vector<ll> a(n);
    for(int i=0;i<n;i++) {
      cin >> a[i];
    }
    ll c = -1;
    ll m = INF;
    bool f = true;
    vector<pll> r;
    for(int i=1;i<n;i++) {
      if(a[i] < a[i-1]) {
        if(c == -1) {
          
        }
      } else if(a[i] == a[i-1]) {
        if(c == -1) {
          c = INF;
        } else {
          if(c != INF) {
            cout << -1 << endl;
            f = false;
            break;
          }
        }
      } else {
        if(c == -1) {
          c = a[i] - a[i-1];
        } else {
          ll k = a[i] - a[i-1];
          if(c != k) {
            cout << -1 << endl;
            f = false;
            break;
          }
        }
      }
    }
    if(!f) continue;
    if(c == INF) {
      cout << 0 << endl;
    }
  }
  return 0;
}
