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
string ans;
bool solve(string s, string t, int n) {
  if(s[0] == 'o') {
    if(t[0] == 'S') t = t[1] + t;
    else {
      if(t[1] == 'S') t = "W" + t;
      else t = "S" + t;
    }
  } else {
    if(t[0] == 'S') {
      if(t[1] == 'S') t = "W" + t;
      else t = "S" + t;
    } else t = t[1] + t;
  }
  for(int i=1;i<n;i++) {
    int len = t.length();
    if(s[i] == 'o') {
      if(t[len-1] == 'S') t += t[len-2];
      else {
        if(t[len-2] == 'S') t += "W";
        else t += "S";
      }
    } else {
      if(t[len-1] == 'S') {
        if(t[len-2] == 'S') t += "W";
        else t += "S";
      } else t += t[len-2];
    }
  }
  // cout << t << endl;
  // cout << t.substr(1, n) << endl;
  // cout << (t[0] == t[l-2] && t[1] == t[l-1]) << endl;
  int l = t.length();
  if(t[0] == t[l-2] && t[1] == t[l-1]) {
    ans = t.substr(1, n);
    return true;
  } else return false;
}
int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  string s;
  cin >> s;
  string v[2] = {"S", "W"};
  bool f = false;
  for(int i=0;i<2;i++) {
    for(int j=0;j<2;j++) {
      string t = v[i] + v[j];
      f = solve(s, t, n);
      if(f) break;
    }
    if(f) break;
  }
  if(f) cout << ans << endl;
  else cout << -1 << endl;
  return 0;
}
