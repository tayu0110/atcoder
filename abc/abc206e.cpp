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
  int l, r;
  cin >> l >> r;
  vector<int> sp(r+1, -1);
  vector<vector<ll>> p(r+1);
  for(int i=2;i<=r;i++) {
    if(sp[i] < 0) {
      for(int j=1;j*i<=r;j++) sp[i*j] = i;
    }
  }
  for(int i=2;i<=r;i++) {
    int k = i;
    set<int> ck;
    while(sp[k] != k) {
      if(ck.find(sp[k]) == ck.end()) p[i].push_back(sp[k]);
      ck.insert(sp[k]);
      k /= sp[k];
    }
    if(ck.find(k) == ck.end()) p[i].push_back(k);
    sort(p[i].begin(), p[i].end());
  }
  ll ans = 0;
  for(int i=l;i<=r;i++) {
    if(i < 2) continue;
    ll t = 0;
    vector<ll>& q = p[i];
    int sz = q.size();
    for(int b=1;b<(1<<sz);b++) {
      int cnt = 0;
      int k = 1;
      for(int j=0;j<sz;j++) {
        if((1<<j) & b) {
          cnt++;
          k *= q[j];
        }
      }
      if(k == 1) continue;
      ll s = r/k - (i-1)/k;
      if(cnt % 2 == 0) s *= -1;
      t += s;
    }
    t -= r/i;
    ans += t;
  }
  cout << 2*ans << endl;
  return 0;
}
