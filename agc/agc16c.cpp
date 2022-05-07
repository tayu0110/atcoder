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

using namespace std;

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

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int h, w, s, t;
  cin >> h >> w >> s >> t;
  if(h % s == 0 && w % t == 0) {
    cout << "No" << endl;
    return 0;
  }
  int diff = (h / s) * (w / t);
  int d = h * w - diff * s * t;
  int k = diff / d + 1;
  vector<vector<int>> b(s, vector<int>(t, 1));
  int sum = 0;
  for(int i=0;i<s;i++) {
    for(int j=0;j<t;j++) {
      if(i < h % s || j < w % t) b[i][j] = k;
      if(i == s-1 && j == t-1) b[i][j] = -sum-1;
      sum += b[i][j];
    }
  }
  cout << "Yes" << endl;
  vector<vector<int>> a(h, vector<int>(w, 0));
  for(int i=0;i<h;i+=s) {
    for(int j=0;j<w;j+=t) {
      for(int k=0;k<s;k++) for(int l=0;l<t;l++) {
        if(i+k >= h || j+l >= w) continue;
        a[i+k][j+l] = b[k][l];
      }
    }
  }
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      if(j) cout << " "; cout << a[i][j];
    }
    cout << endl;
  }
  return 0;
}
