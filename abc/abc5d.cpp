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
  vector<vector<int>> d(n, vector<int>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    cin >> d[i][j];
  }
  vector<vector<int>> ts(n+1, vector<int>(n+1));
  for(int i=1;i<n+1;i++) for(int j=0;j<n;j++) ts[i][j+1] = ts[i][j] + d[i-1][j];
  for(int i=0;i<n;i++) for(int j=0;j<n+1;j++) ts[i+1][j] += ts[i][j];
  vector<int> ans(n*n+1);
  for(int a=1;a<n*n+1;a++) {
    int mx = 0;
    for(int i=1;i*i<=a;i++) {
      if(a % i != 0) continue;
      int j = a / i;
      for(int k=1;k<n+1;k++) for(int l=1;l<n+1;l++) {
        int s = -1, t = -1;
        if(k-i >= 0 && l-j >= 0) s = ts[k][l] - ts[k-i][l] - ts[k][l-j] + ts[k-i][l-j];
        if(k-j >= 0 && l-i >= 0) t = ts[k][l] - ts[k-j][l] - ts[k][l-i] + ts[k-j][l-i];
        mx = max(mx, max(s, t));
      }
    }
    ans[a] = mx;
  }
  for(int i=0;i<n*n;i++) ans[i+1] = max(ans[i+1], ans[i]);
  int q;
  cin >> q;
  vector<int> res;
  while(q--) {
    int p;
    cin >> p;
    res.push_back(ans[p]);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
