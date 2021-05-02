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

struct SegmentTree {
  int sz;
  vector<int> t;
  SegmentTree(int n) {
    sz = 1;
    while(sz < n) sz *= 2;
    t.assign(2 * sz - 1, 0);
  }
  void update(int idx, int val) {
    idx += sz - 1;
    t[idx] = val;
    while(idx > 0) {
      idx = (idx - 1) / 2;
      if(t[idx] > val) break;
      t[idx] = val;
    }
    return;
  }
  int getMax(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res = max(res, getMax(l, r, 2*now+1, a, (a+b)/2));
    res = max(res, getMax(l, r, 2*now+2, (a+b)/2, b));
    return res;
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<bool> ck(n+1, false);
  set<int> p;
  vector<int> d(n+1);
  for(int i=2;i<n+1;i++) {
    if(!ck[i]) p.insert(i);
    for(int j=2;j*i<n+1;j++) {
      ck[i*j] = true;
    }
    int k = i;
    for(auto e : p) {
      while(k % e == 0) {
        k /= e;
        d[e]++;
      }
    }
  }
  int ans = 0;
  for(int i=2;i<n+1;i++) {
    if(d[i] >= 74) ans++;
    for(int j=i+1;j<n+1;j++) {
      if(d[i] >= 24 && d[j] >= 2) ans++;
      if(d[i] >= 2 && d[j] >= 24) ans++;
      if(d[i] >= 14 && d[j] >= 4) ans++;
      if(d[i] >= 4 && d[j] >= 14) ans++;
      for(int k=j+1;k<n+1;k++) {
        if(d[i] >= 2 && d[j] >= 4 && d[k] >= 4) ans++;
        if(d[i] >= 4 && d[j] >= 4 && d[k] >= 2) ans++;
        if(d[i] >= 4 && d[j] >= 2 && d[k] >= 4) ans++;
      }
    }
  }
  cout << ans << endl;
  return 0;
}
