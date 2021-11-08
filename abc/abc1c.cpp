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
  int deg, dis;
  cin >> deg >> dis;
  string dir;
  int w;
  if(deg <= 112 || 3487 < deg) dir = "N";
  else if(deg <= 337) dir = "NNE";
  else if(deg <= 562) dir = "NE";
  else if(deg <= 787) dir = "ENE";
  else if(deg <= 1012) dir = "E";
  else if(deg <= 1237) dir = "ESE";
  else if(deg <= 1462) dir = "SE";
  else if(deg <= 1687) dir = "SSE";
  else if(deg <= 1912) dir = "S";
  else if(deg <= 2137) dir = "SSW";
  else if(deg <= 2362) dir = "SW";
  else if(deg <= 2587) dir = "WSW";
  else if(deg <= 2812) dir = "W";
  else if(deg <= 3037) dir = "WNW";
  else if(deg <= 3262) dir = "NW";
  else if(deg <= 3487) dir = "NNW";
  dis = dis * 10 / 6;
  dis = round((double)dis/10);
  if(dis <= 2) w = 0;
  else if(dis <= 15) w = 1;
  else if(dis <= 33) w = 2;
  else if(dis <= 54) w = 3;
  else if(dis <= 79) w = 4;
  else if(dis <= 107) w = 5;
  else if(dis <= 138) w = 6;
  else if(dis <= 171) w = 7;
  else if(dis <= 207) w = 8;
  else if(dis <= 244) w = 9;
  else if(dis <= 284) w = 10;
  else if(dis <= 326) w = 11;
  else w = 12;
  if(w == 0) dir = "C";
  cout << dir << " " << w << endl;
  return 0;
}
