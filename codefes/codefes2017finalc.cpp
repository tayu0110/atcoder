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
  if(n > 23) {
    cout << 0 << endl;
    return 0;
  }
  vector<int> d;
  vector<int> c;
  vector<bool> ck(24, false);
  ck[0] = true;
  for(int i=0;i<n;i++) {
    int k;
    cin >> k;
    if(k == 0) {
      cout << 0 << endl;
      return 0;
    }
    if(ck[k]) {
      int nd = 24 - k;
      if(ck[nd]) {
        cout << 0 << endl;
        return 0;
      } else {
        ck[nd] = true;
        c.push_back(nd);
        c.push_back(k);
        for(auto it = d.begin(); it != d.end(); it++) {
          if(*it == k) {
            d.erase(it);
            break;
          }
        }
      }
    } else {
      ck[k] = true;
      d.push_back(k);
    }
  }
  int m = d.size();
  int ans = 0;
  for(int i=0;i<(1<<m);i++) {
    vector<int> t = c;
    int mn = inf;
    for(int j=0;j<m;j++) {
      if(i & (1<<j)) t.push_back(d[j]);
      else t.push_back(24-d[j]);
    }
    t.push_back(0);
    t.push_back(24);
    for(int j=0;j<t.size();j++) {
      for(int k=j+1;k<t.size();k++) {
        mn = min(mn, abs(t[j]-t[k]));
      }
    }
    ans = max(ans, mn);
  }
  cout << ans << endl;
  return 0;
}
