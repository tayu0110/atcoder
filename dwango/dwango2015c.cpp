#include <iostream>
#include <iomanip>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
#include <tuple>
#include <map>
#include <queue>
#include <deque>
#include <set>
#include <stack>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>
#include <cassert>

#include <atcoder/all>

using namespace std;
using namespace atcoder;

#define DEBUG(var) cerr << #var << ": " << (var) << " "
#define DEBUG_EN(var) cerr << #var << ": " << (var) << endl

using ll = long long;
using ld = long double;
using pii = pair<int, int>;
using pll = pair<ll, ll>;
using Graph = vector<vector<int>>;
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;
const ld PI = acos(-1);

int n;
vector<double> e;
vector<vector<double>> t;
double rec(int now) {
  if(now == 1) return 0;
  if(e[now] >= -0.1) return e[now];
  double res = 0;
  for(int i=1;i<now;i++) res += t[now][i] * (rec(i) + 1);
  res += t[now][now];
  res /= 1 - t[now][now];
  return e[now] = res;
}

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n;
  vector<vector<vector<double>>> p(n+1, vector<vector<double>>(n+1, vector<double>(n+1)));
  p[0][0][0] = 1;
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      for(int k=0;k<n;k++) {
        p[i+1][j][k] += p[i][j][k] / 3;
        p[i][j+1][k] += p[i][j][k] / 3;
        p[i][j][k+1] += p[i][j][k] / 3;
      }
    }
  }
  t.assign(n+1, vector<double>(n+1, 0));
  for(int i=0;i<n+1;i++) {
    for(int j=0;j<n+1;j++) {
      for(int k=0;k<n+1;k++) {
        if(i+j+k > n) continue;
        int mn = inf;
        int zero = !i + !j + !k;
        if(zero == 2) {
          t[i+j+k][i+j+k] += p[i][j][k];
          continue;
        }
        if(i) mn = min(i, mn);
        if(j) mn = min(j, mn);
        if(k) mn = min(k, mn);
        if(i == j && j == k) t[i+j+k][i+j+k] += p[i][j][k];
        else t[i+j+k][mn] += p[i][j][k];
      }
    }
  }
  e.assign(n+1, -1);
  rec(n);
  cout << e[n] << endl;
  return 0;
}
