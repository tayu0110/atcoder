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
  Edge(const Edge& e) : to(e.to), weight(e.weight) {}
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
template<class T> void print_with_space(T p) { for(auto e : p) cout << e << " "; cout << endl; }

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;
int check(int u, int l, vector<vector<int>> &v, vector<vector<int>> &h) {
  int n = v.size(), m = v[0].size();
  vector<vector<int>> memo(n, vector<int>(m, -1));
  int now = 0;
  now += 4 - (h[u][l+2] - h[u][l] + h[u+4][l+2] - h[u+4][l]);
  for(int i=1;i<4;i++) now += 2 - (h[u+i][l+3] - h[u+i][l]);
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  while(t--) {
    int n, m;
    cin >> n >> m;
    vector<string> a(n);
    for(int i=0;i<n;i++) cin >> a[i];
    vector<vector<int>> v(n, vector<int>(m, 0)), h(n, vector<int>(m, 0));
    for(int i=0;i<n;i++) {
      for(int j=0;j<m;j++) {
        h[i][j] += (a[i][j] == '1');
        if(j < m-1) h[i][j+1] += h[i][j];
        v[i][j] += (a[i][j] == '1');
        if(i < n-1) v[i+1][j] += v[i][j];
      }
    }
  }
  return 0;
}
