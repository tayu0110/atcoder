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
};

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using pdd = pair<double, double>;
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

bool check(pdd A, pdd B, pdd S, pdd T) {
  double ax = A.first, ay = A.second;
  double bx = B.first, by = B.second;
  double sx = S.first, sy = S.second;
  double tx = T.first, ty = T.second;
  double k = (sx - ax) * (ay - by) - (ax - bx) * (sy - ay);
  double l = (tx - ax) * (ay - by) - (ax - bx) * (ty - ay);
  if(k * l > 0) return false;
  k = (ax - sx) * (sy - ty) - (sx - tx) * (ay - sy);
  l = (bx - sx) * (sy - ty) - (sx - tx) * (by - sy);
  if(k * l > 0) return false;
  return true;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int x, y, a, b;
  cin >> x >> y >> a >> b;
  pdd A = {x, y};
  pdd B = {a, b};
  int n;
  cin >> n;
  vector<pdd> l(n);
  for(int i=0;i<n;i++) {
    cin >> l[i].first >> l[i].second;
  }
  l.push_back(l[0]); 
  int p = 0;
  for(int i=0;i<n;i++) {
    pdd X = l[i];
    pdd Y = l[i+1];
    if(check(A, B, X, Y)) p++;
  }
  p /= 2;
  cout << p + 1 << endl;
  return 0;
}
