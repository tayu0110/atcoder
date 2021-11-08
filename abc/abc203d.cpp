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
#include<cassert>

using namespace std;

#define DEBUG(var) cout << #var << ": " << var << " ";
#define DEBUG_EN(var) cout << #var << ": " << var << endl;

struct Edge {
  int to;
  long long weight;
  Edge() : to(0), weight(0) {}
  Edge(int to, long long weight) : to(to), weight(weight) {}
  Edge(const Edge& e) {
    to = e.to;
    weight = e.weight;
  }
  bool operator>(const Edge &e) const { return weight > e.weight; }
  bool operator<(const Edge &e) const { return weight < e.weight; }
  bool operator==(const Edge &e) const { return weight == e.weight; }
  bool operator<=(const Edge &e) const { return weight <= e.weight; }
  bool operator>=(const Edge &e) const { return weight >= e.weight; }
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
using weightedGraph = vector<vector<Edge>>;
using heap = priority_queue<int, vector<int>, greater<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<vector<ll>> a(n, vector<ll>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) cin >> a[i][j];
  ll l = -1, r = 1001001001;
  while(r-l > 1) {
    ll m = (r+l) / 2;
    vector<vector<int>> t(n+1, vector<int>(n+1, 0));
    for(int i=0;i<n;i++) for(int j=0;j<n;j++) t[i+1][j+1] = (a[i][j] > m ? 1 : 0);
    for(int i=1;i<n+1;i++) for(int j=0;j<n;j++) t[i][j+1] += t[i][j];
    for(int i=0;i<n;i++) for(int j=1;j<n+1;j++) t[i+1][j] += t[i][j];
    bool f = false;
    for(int i=k;i<n+1;i++) for(int j=k;j<n+1;j++) {
      ll s = t[i][j] - t[i-k][j] - t[i][j-k] + t[i-k][j-k];
      if(s < k*k / 2 + 1) f = true;
    }
    if(f) r = m;
    else l = m;
  }
  cout << r << endl;
  return 0;
}
