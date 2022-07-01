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


int isPal(string s) {
  int len = s.length();
  int res = 0;
  for(int i=0;i<len;i++) {
    if(i > len-i-1) break;
    if(s[i] != s[len-i-1]) res++;
  }
  return res;
}

int dfs(string s, int lev) {
  cout << "lev: " << lev << " s: " << s << endl; 
  if(lev == 1) {
    if(s.length() == 0 || !isPal(s)) return 0;
    else {
      cout << "impossible" << endl;
      exit(0);
    }
  }
  int len = s.length();
  if(len == 1 && lev != 2) {
    cout << "impossible" << endl;
    exit(0);
  }
  if(len == 1 && lev == 2) {
    return 0;
  }
  int g;
  if(len & 1) g = len/2 + 1;
  else g = len/2;
  int res = 0;
  for(int i=0;i<len/2;i++) {
    char &ff = s[i], &fb = s[len/2-i-1], &bf = s[g+i], &bb = s[len-i-1];
    if(i > len/2 - i- 1) break;
    if(ff != bf) {
      if(ff == fb && bf == bb) {
        bf = ff;
        res++;
        if(i != len/2-i-1) {
          bb = ff;
          res++;
        }
      } else if(ff == fb) {
        bf = ff;
        res++;
        if(bf != bb) {
          bb = ff;
          res++;
        }
      } else if(bf == bb) {
        ff = bf;
        res++;
        if(fb != bf){
          fb = bf;
          res++;
        }
      } else {
        fb = ff;
        bf = ff;
        res += 2;
        if(bb != ff) {
          bb = ff;
          res++;
        }
      }
    } else {
      if(ff == fb) {
        if(bf != bb) {
          bb = ff;
          res++;
        }
      } else if(bf == bb) {
        if(ff != fb) {
          fb = ff;
          res++;
        }
      }
    }
  }
  string f = s.substr(0, len/2);
  res += dfs(f, lev-1);
  return res;
}

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int k;
  cin >> k;
  string s;
  cin >> s;
  int res = dfs(s, k);
  cout << res << endl;
  return 0;
}
