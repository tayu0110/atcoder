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

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<int>> c(1010, vector<int>(1010, 0));
  for(int i=0;i<n;i++) {
    int lx, ly, rx, ry;
    cin >> lx >> ly >> rx >> ry;
    c[ly][lx]++;c[ry][lx]--;c[ly][rx]--;c[ry][rx]++;
  }
  for(int i=0;i<1005;i++) for(int j=0;j<1005;j++) {
    c[i][j+1] += c[i][j];  
  }
  for(int j=0;j<1005;j++) for(int i=0;i<1005;i++) {
    c[i+1][j] += c[i][j];
  }
  vector<int> ans(n+1);
  for(int i=0;i<1005;i++) for(int j=0;j<1005;j++) {
    if(c[i][j] >= 0) ans[c[i][j]]++;
  }
  for(int i=1;i<n+1;i++) cout << ans[i] << endl;
  return 0;
}
