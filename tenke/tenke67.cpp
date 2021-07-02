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
  string n;
  ll k;
  cin >> n >> k;
  if(n == "0") {
    cout << 0 << endl;
    return 0;
  }
  while(k--) {
    ll nn = 0;
    ll t = 1;
    while(n != "") {
      ll v = n[n.length()-1] - '0';
      nn += t * v;
      t *= 8;
      n.pop_back();
    }
    t = 1;
    ll m = 0;
    while(nn != 0) {
      ll s = nn % 9;
      if(s == 8) s = 5;
      m += s * t;
      t *= 10;
      nn /= 9;
    }
    while(m != 0) {
      n = char((m%10) + '0') + n;
      m /= 10;
    }
  }
  cout << n << endl;
  return 0;
}
