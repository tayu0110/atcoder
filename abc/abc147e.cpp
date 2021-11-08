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
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<vector<int>> a(h, vector<int>(w, 0)), b(h, vector<int>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> a[i][j];
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> b[i][j];
  vector<vector<int>> t(h, vector<int>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) t[i][j] = abs(a[i][j] - b[i][j]);
  vector<vector<vector<int>>> d(h, vector<vector<int>>(w, vector<int>(13000, 0)));
  d[0][0][0] = 1;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    for(int k=0;k<d[0][0].size();k++) {
      if(!d[i][j][k]) continue;
      int s1 = abs(k + t[i][j]);
      int s2 = abs(k - t[i][j]);
      if(i+1 < h) d[i+1][j][s1] = 1, d[i+1][j][s2] = 1;
      if(j+1 < w) d[i][j+1][s1] = 1, d[i][j+1][s2] = 1;
    }
  }
  int mn = inf;
  for(int i=0;i<13000;i++) {
    if(d[h-1][w-1][i]) {
      int s1 = abs(i + t[h-1][w-1]);
      int s2 = abs(i - t[h-1][w-1]);
      mn = min(mn, s1);
      mn = min(mn, s2);
    }
  }
  cout << mn << endl;
  return 0;
}
