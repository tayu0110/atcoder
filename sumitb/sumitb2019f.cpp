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
  ll s, t, a1, a2, b1, b2;
  cin >> s >> t >> a1 >> a2 >> b1 >> b2;
  a1 *= s, b1 *= s;
  a2 *= t, b2 *= t;
  if(a1 > b1) {
    swap(a1, b1);
    swap(a2, b2);
  }
  ll k = a1 + a2, l = b1 + b2;
  if(k < l) {
    cout << 0 << endl;
    return 0;
  }
  if(k == l) {
    cout << "infinity" << endl;
    return 0;
  }
  ll ans = (b1-a1) / (k-l);
  if(ans * (k-l) == (b1-a1)) {
    cout << 2*ans << endl;
  } else {
    ans++;
    cout << 2*ans-1 << endl;
  }
  return 0;
}