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
  int h, w;
  cin >> h >> w;
  vector<vector<int>> p(h, vector<int>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> p[i][j];
  int ans = 0;
  for(int i=1;i<(1<<h);i++) {
    vector<vector<int>> q;
    for(int j=0;j<h;j++) if((1<<j) & i) q.push_back(p[j]);
    int a = q.size(), b = 0;
    map<int, int> mp;
    for(int j=0;j<w;j++) {
      int l = q[0][j];
      bool f = true;
      for(int k=0;k<a;k++) f &= (q[k][j] == l);
      if(f) mp[l]++;
    }
    int check;
    for(auto e : mp) {
      if(b < e.second) {
        b = max(b, e.second);
        check = e.first;
      }
    }
    ans = max(ans, a * b);
  }
  cout << ans << endl;
  return 0;
}
