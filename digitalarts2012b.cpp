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
  string c;
  cin >> c;
  set<char> ck;
  map<char, int> mp;
  int hash = 0;
  for(int i=0;i<c.size();i++) {
    ck.insert(c[i]);
    mp[c[i]]++;
    hash += c[i] - 'a' + 1;
  }
  if(ck.size() == 1 && mp['a'] == 1) {
    cout << "NO" << endl;
    return 0;
  }
  if(ck.size() == 1 && mp['z'] == 20) {
    cout << "NO" << endl;
    return 0;
  }
  bool f = false;
  string ans;
  int n = 0;
  while(n != hash) {
    int len = ans.length();
    if(hash - n < 27) {
      int k = hash - n - 1;
      char t = k + 'a';
      if(f) {
        ans += t;
        n += k+1;
      } else {
        if(c[len] == t) {
          if(t == 'a') {
            ans += "b";
            n += 2;
          } else {
            ans += "a";
            n++;
          }
          f = true;
        } else {
          ans += t;
          n += k+1;
        }
      }
    } else {
      if(f) {
        ans += "z";
        n += 26;
      } else {
        if(c[len] == 'z') {
          ans += "y";
          n += 25;
        } else {
          ans += "z";
          n += 26;
        }
        f = true;
      }
    }
  }
  cout << ans << endl;
  return 0;
}
