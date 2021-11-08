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
  int t;
  cin >> t;
  while(t--) {
    ll s[3], m;
    cin >> s[0] >> s[1] >> s[2] >> m;
    sort(s, s+3);
    ll a = s[0], b = s[1], c = s[2];
    if(a+b+c-3 < m) {
      cout << "NO" << endl;
      continue;
    }
    if(c-a < 2) {
      cout << "YES" << endl;
      continue;
    }
    ll p = a + b;
    if(abs(p-c) < 2) {
      cout << "YES" << endl;
      continue;
    }
    if(p < c) {
      ll d = c - p - 1;
      if(d <= m) {
        cout << "YES" << endl;
        continue;
      }
    } else {
      ll d = p - c - 1;
      if(d <= m) {
        cout << "YES" << endl;
        continue;
      }
    }
    if(b + c - 2*a - 2 <= m) {
      cout << "YES" << endl;
      continue;
    }
    cout << "NO" << endl;
  }
  return 0;
}
