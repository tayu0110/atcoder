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
  int n, q;
  cin >> n >> q;
  string s;
  cin >> s;
  vector<pair<char, char>> p(q);
  for(int i=0;i<q;i++) {
    char t, d;
    cin >> t >> d;
    p[i] = {t, d};
  }
  int lhs = n, rhs = 0;
  int l = 0, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    char c = s[m];
    int now = m;
    for(int i=0;i<q;i++) {
      char t = p[i].first, d = p[i].second;
      if(c != t) continue;
      if(d == 'L') now--;
      else now++;
      if(now < 0 || n <= now) break;
      c = s[now];
    }
    if(now < 0) l = m;
    else r = m;
  }
  lhs = l;
  l = 0, r = n;
  while(r-l > 1) {
    int m = (r+l) / 2;
    char c = s[m];
    int now = m;
    for(int i=0;i<q;i++) {
      char t = p[i].first, d = p[i].second;
      if(c != t) continue;
      if(d == 'L') now--;
      else now++;
      if(now < 0 || n <= now) break;
      c = s[now];
    }
    if(now >= n) r = m;
    else l = m;
  }
  rhs = r;
  cout << rhs-lhs-1 << endl;
  return 0;
}
