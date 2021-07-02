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
ll bin(vector<ll>& p) {
  sort(p.begin(), p.end());
  ll l = -100100100, r = 100100100;
  set<pll> ck;
  auto f = [&](ll s) {
    ll sum = 0;
    for(int i=0;i<p.size();i++) sum += abs(s - p[i]);
    return sum;
  };
  while(r - l > 1) {
    if(ck.find({l, r}) != ck.end()) {
      ll k = f(l+1), m = f(r-1);
      ll tl = f(l), tr = f(r);
      if(k < tl) l++;
      else if(m < tr) r--;
      else break;
    }
    ck.insert({l, r});
    ll nl = (2*l+r) / 3;
    ll nr = (l+2*r) / 3;
    ll tl = f(nl), tr = f(nr);
    if(tl < tr) r = nr;
    else l = nl;
  }
  ll tl = f(l), tr = f(r);
  if(tl < tr) return tl;
  else return tr;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  ll sx = 0, sy = 0;
  vector<ll> x(n), y(n);
  for(int i=0;i<n;i++) {
    cin >> x[i] >> y[i];
  }
  cout << bin(x) + bin(y) << endl;
  return 0;
}
