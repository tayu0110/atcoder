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
  void init(int idx, ll val) {
    idx += sz-1;
    t[idx] = val;
    return;
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
  void update(int l, int r, ll val, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(r < a || l > b) return;
    if(a <= l && r <= b) {
      t[now] += val;
      update(l, r, val, now*2+1, a, (a+b)/2);
      update(l, r, val, now*2+2, (a+b)/2, b);
    }
    return;
  }
  int getSum(int l, int r, int now=0, int a=0, int b=-1) {
    if(now >= t.size()) return 0;
    if(b < 0) b = sz;
    if(l < 0) l = 0;
    if(r > sz) r = sz;
    if(l > b || r < a) return 0;
    if(l <= a && r >= b) return t[now];
    int res = 0;
    res += getSum(l, r, 2*now+1, a, (a+b)/2);
    res += getSum(l, r, 2*now+2, (a+b)/2, b);
    return res;
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
  ll getElement(int idx) {
    if(idx == 0) return t[0];
    ll res = t[idx];
    return res += getElement((idx-1)/2);
  }
  ll operator[](int idx) {
    idx += sz-1;
    return getElement(idx);
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  string s;
  cin >> s;
  vector<SegmentTree> st(26, SegmentTree(n));
  for(int i=0;i<n;i++) {
    int t = s[i] - 'a';
    st[t].update(i, i+1, 1);
    // cout << i << " " << s[i] << " " << st[t].getSum(0, n) << endl;
  }
  for(int i=0;i<26;i++) {
    // cout << (char)('a'+i) << " " << st[i].getSum(0, n) << endl;;
  }
  int q;
  cin >> q;
  while(q--) {
    int t;
    cin >> t;
    if(t == 1) {
      int i;
      char c;
      cin >> i >> c;
      i--;
      int k = c - 'a';
      int l = s[i] - 'a';
      s[i] = c;
      st[k].update(i, i+1, 1);
      st[l].update(i, i+1, -1);
    } else {
      int l, r;
      cin >> l >> r;
      l--;
      int res = 0;
      for(int j=0;j<26;j++) {
        if(st[j].getSum(l, r)) res++;
      }
      cout << res << endl;
    }
  }
  return 0;
}
