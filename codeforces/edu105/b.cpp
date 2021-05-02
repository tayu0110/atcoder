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
  int t;
  cin >> t;
  for(int k = 0; k < t; k++) {
    int n, u, r, d, l;
    cin >> n >> u >> r >> d >> l;
    if(u == n && d == n) {
      if(l > 1 && r > 1) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(l == n && r == n) {
      if(u > 1 && d > 1) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(max(u, d) == n && min(u, d) == n-1) {
      if(max(l, r) > 1 && min(l, r) > 0) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(max(l, r) == n && min(l, r) == n-1) {
      if(max(u, d) > 1 && min(u, d) > 0) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(u == n-1 && d == n-1) {
      if((l > 0 && r > 0) || (max(l, r) > 1 && min(l, r) >= 0)) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(l == n-1 && r == n-1) {
      if((u > 0 && d > 0) || (max(u, d) > 1 && min(u, d) >= 0)) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(u == n || d == n) {
      if(l == n || r == n) {
        if(min(u, d) > 0 && min(l, r) > 0) cout << "YES" << endl;
        else cout << "NO" << endl;
      } else {
        if(l > 0 && r > 0) cout << "YES" << endl;
        else cout << "NO" << endl;
      }
    } else if(l == n || r == n) {
      if(u > 0 && d > 0) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(u == n-1 || d == n-1) {
      if(l > 0 || r > 0) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else if(l == n-1 || r == n-1) {
      if(u > 0 || d > 0) cout << "YES" << endl;
      else cout << "NO" << endl;
    } else {
      cout << "YES" << endl;
    }
  }
  return 0;
}
