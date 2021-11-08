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
using heap = priority_queue<pair<int, pii>, vector<pair<int, pii>>, greater<pair<int, pii>>>;
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
  int r, c;
  cin >> r >> c;
  vector<vector<int>> a(r, vector<int>(c-1));
  vector<vector<int>> b(r-1, vector<int>(c));
  for(int i=0;i<r;i++) for(int j=0;j<c-1;j++) cin >> a[i][j];
  for(int i=0;i<r-1;i++) for(int j=0;j<c;j++) cin >> b[i][j];
  vector<vector<int>> memo(r, vector<int>(c, inf)), d(r, vector<int>(c, -1));
  vector<set<pii>> ck(2500010);
  memo[0][0] = 0;
  ck[0].insert({0, 0});
  int cnt = 0;
  auto f = [&](int ar, int ac, int dr, int dc, int ab) {
    int newr = ar+dr, newc = ac+dc;
    if(memo[newr][newc] > cnt + ab) {
      if(memo[newr][newc] != inf) ck[memo[newr][newc]].erase({newr, newc});
      ck[cnt + ab].insert({newr, newc});
      memo[newr][newc] = cnt + ab;
    }
  };
  while(cnt < ck.size()) {
    if(ck[cnt].empty()) {
      cnt++;
      continue;
    }
    auto p = *ck[cnt].begin();
    ck[cnt].erase(p);
    int nr = p.first, nc = p.second;
    d[nr][nc] = cnt;
    if(nr == r-1 && nc == c-1) break;
    if(nc+1 < c && d[nr][nc+1] < 0) f(nr, nc, 0, 1, a[nr][nc]);
    if(nc > 0 && d[nr][nc-1] < 0) f(nr, nc, 0, -1, a[nr][nc-1]);
    if(nr+1 < r && d[nr+1][nc] < 0) f(nr, nc, 1, 0, b[nr][nc]);
    for(int j=1;j<nr+1;j++) if(d[nr-j][nc] < 0) f(nr, nc, -j, 0, 1+j);
    if(ck[cnt].empty()) cnt++;
  }
  cout << d[r-1][c-1] << endl;
  return 0;
}
