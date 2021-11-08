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

const ll BIL = 1e9;
const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = 3.141592653589793238462643383;

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int r, c, k;
  cin >> r >> c >> k;
  vector<string> s(r);
  for(int i=0;i<r;i++) cin >> s[i];
  int ans = 0;
  vector<vector<int>> lhs(r, vector<int>(c, 0));
  vector<vector<int>> rhs(r, vector<int>(c, 0));
  for(int i=0;i<r;i++) {
    for(int j=0;j<c;j++) if(s[i][j] == 'o') lhs[i][j] = (j-1 >= 0 ? lhs[i][j-1] : 0) + 1;
    for(int j=c-1;j>=0;j--) if(s[i][j] == 'o') rhs[i][j] = (j+1 < c ? rhs[i][j+1] : 0) + 1;
  }
  for(int i=k-1;i<r-k+1;i++) {
    for(int j=k-1;j<c-k+1;j++) {
      bool f = true;
      for(int l=i-(k-1);l<i+k;l++) {
        int diff = abs(i-l);
        int t = (k-1-diff)+1;
        if(lhs[l][j] >= t && rhs[l][j] >= t) continue;
        f = false;
        break;
      }
      if(f) ans++;
    }
  }
  cout << ans << endl;
  return 0;
}
