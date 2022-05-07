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

#define DEBUG(var) cerr << #var << ": " << var << " "
#define DEBUG_EN(var) cerr << #var << ": " << var << endl

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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w;
  cin >> h >> w;
  vector<vector<ll>> a(h, vector<ll>(w));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> a[i][j];
  for(int i=0;i<h-1;i++) {
    for(int j=0;j<w-1;j++) {
      if(i < h-2) {
        ll t = a[i][j];
        for(int k=0;k<2;k++) for(int l=0;l<2;l++) a[i+k][j+l] -= t;
      } else {
        if(j < w-2) {
          ll t = (a[i][j] + a[i+1][j]) / 2;
          for(int k=0;k<2;k++) for(int l=0;l<2;l++) a[i+k][j+l] -= t;
        } else {
          vector<ll> t = {a[i][j], a[i+1][j], a[i][j+1], a[i+1][j+1]};
          sort(t.begin(), t.end());
          ll p = 0, q = 0;
          for(int k=0;k<2;k++) for(int l=0;l<2;l++) p += abs(a[i+k][j+l] - t[1]);
          for(int k=0;k<2;k++) for(int l=0;l<2;l++) q += abs(a[i+k][j+l] - t[2]);
          if(p < q) for(int k=0;k<2;k++) for(int l=0;l<2;l++) a[i+k][j+l] - t[1];
          else for(int k=0;k<2;k++) for(int l=0;l<2;l++) a[i+k][j+l] - t[2];
        }
      }
    }
  }
  ll sum = 0;
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) sum += abs(a[i][j]);
  cout << sum << endl;
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(j) cout << " "; cout << a[i][j];
    }
    cout << endl;
  }
  return 0;
}
