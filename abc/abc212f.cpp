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
  long long start;
  long long reach;
  Edge() : to(0), start(0), reach(0) {}
  Edge(int to, long long start, long long reach) : to(to), start(start), reach(reach) {}
  Edge(const Edge& e) {
    to = e.to;
    start = e.start;
    reach = e.reach;
  }
  bool operator>(const Edge &e) const { return start > e.start; }
  bool operator<(const Edge &e) const { return start < e.start; }
  bool operator==(const Edge &e) const { return start == e.start; }
  bool operator<=(const Edge &e) const { return start <= e.start; }
  bool operator>=(const Edge &e) const { return start >= e.start; }
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
  int n, m, q;
  cin >> n >> m >> q;
  weightedGraph t(n);
  for(int i=0;i<m;i++) {
    int a, b;
    ll s, r;
    cin >> a >> b >> s >> r;
    a--;b--;
    t[a].push_back(Edge(b, s, r+1));
  }
  
  return 0;
}
