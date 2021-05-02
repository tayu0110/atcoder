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

ll comb(ll m, ll k) {
  ll t = k;
  ll res = 1;
  for(int i=0;i<k;i++) {
    res *= m;
    m--;
  }
  for(int i=0;i<t;i++) {
    res /= k;
    k--;
  }
  return res;
}

ll dp[105][5][2];

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  string n;
  int k;
  cin >> n >> k;
  int m = n.size();
  dp[0][0][0] = 1;
  for(int i=0;i<m;i++) {
    int t = n[i] - '0';
    for(int j=0;j<k+1;j++) {
      for(int l=0;l<2;l++) {
        for(int p=0;p<10;p++) {
          int a = j;
          int b = l;
          if(p != 0) a++;
          if(a > k) continue;
          if((!l) && (p > t)) continue;
          if(p < t) b = 1;
          dp[i+1][a][b] += dp[i][j][l];
        }
      }
    }
  }
  cout << dp[m][k][0] + dp[m][k][1] << endl;
  return 0;
}
