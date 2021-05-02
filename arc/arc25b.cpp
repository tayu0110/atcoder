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

class TwoDimCumulativeSum {
  int h, w;
  vector<vector<int>> sum;
public:
  TwoDimCumulativeSum(vector<vector<int>> a) {
    h = a.size();
    w = a[0].size();
    sum.assign(h+1, vector<int>(w+1));
    for(int i = 1; i < h+1; i++) for(int j = 1; j < w+1; j++) sum[i][j] = a[i-1][j-1] + sum[i-1][j] + sum[i][j-1] - sum[i-1][j-1];
  }
  int rangeSum(int top, int btm, int lft, int rht) {
    return sum[btm][rht] - sum[top][rht] - sum[btm][lft] + sum[top][lft];
  }
};

int main(int argc,char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<vector<int>> bk(h, vector<int>(w)), wt(h, vector<int>(w));
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if((i+j) % 2 == 1) cin >> bk[i][j];
      else cin >> wt[i][j];
    }
  }
  TwoDimCumulativeSum sb(bk), sw(wt);
  int mx = 0;
  for(int i=0;i<h+1;i++) for(int j=i+1;j<h+1;j++) for(int k=0;k<w+1;k++) for(int l=k+1;l<w+1;l++) {
    int bn = sb.rangeSum(i, j, k, l);
    int wn = sw.rangeSum(i, j, k, l);
    if(bn == wn) mx = max(mx, (j-i) * (l-k));
  }
  cout << mx << endl;
  // vector<vector<int>> sb(h+1, vector<int>(w+1)), sw(h+1, vector<int>(w+1));
  // for(int i=1;i<h+1;i++) for(int j=1;j<w+1;j++) {
  //   sb[i][j] = bk[i-1][j-1] + sb[i-1][j] + sb[i][j-1] - sb[i-1][j-1];
  //   sw[i][j] = wt[i-1][j-1] + sw[i-1][j] + sw[i][j-1] - sw[i-1][j-1];
  // }
  // int mx = 0;
  // for(int i=0;i<h+1;i++) {
  //   for(int j=i+1;j<h+1;j++) {
  //     for(int k=0;k<w+1;k++) {
  //       for(int l=0;l<w+1;l++) {
  //         int bn = sb[j][l] - sb[i][l] - sb[j][k] + sb[i][k];
  //         int wn = sw[j][l] - sw[i][l] - sw[j][k] + sw[i][k];
  //         if(bn == wn) {
  //           mx = max(mx, (j-i) * (l-k));
  //         }
  //       }
  //     }
  //   }
  // }
  // cout << mx << endl;
  return 0;
}
