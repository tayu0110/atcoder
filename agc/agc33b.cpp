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

bool check(vector<int> &t, int h, int w) {
  if(t[0] < 1) {
      cout << "NO" << endl;
      // cout << "reach1" << endl;
      return true;
    } else if(t[1] > h) {
      // cout << "reach2" << endl;
      cout << "NO" << endl;
      return true;
    } else if(t[2] < 1) {
      // cout << "reach3" << endl;
      cout << "NO" << endl;
      return true;
    } else if(t[3] > w) {
      // cout << "reach4" << endl;
      cout << "NO" << endl;
      return true;
    }
    return false;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w, n;
  cin >> h >> w >> n;
  int sr, sc;
  cin >> sr >> sc;
  string s, t;
  cin >> s >> t;
  vector<int> k(4);
  k[0] = k[1] = sr;
  k[2] = k[3] = sc;
  for(int i=0;i<n;i++) {
    char c = s[i], d = t[i];
    if(c == 'U') k[0]--;
    else if(c == 'D') k[1]++;
    else if(c == 'L') k[2]--;
    else if(c == 'R') k[3]++;
    bool f = check(k, h, w);
    if(f) return 0;
    if(d == 'D') k[0] = min(k[0]+1, h);
    else if(d == 'U') k[1] = max(k[1]-1, 1);
    else if(d == 'R') k[2] = min(k[2]+1, w);
    else if(d == 'L') k[3] = max(k[3]-1, 1);
    f = check(k, h, w);
    if(f) return 0;
  }
  cout << "YES" << endl;
  return 0;
}
