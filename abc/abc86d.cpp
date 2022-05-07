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
const ld PI = 3.141592653589793238462643383;
vector<vector<int>> t;
vector<vector<int>> sum;
int get_sum(int x1, int y1, int x2, int y2) {
  int res = sum[x1][y1] - sum[x1][y2] - sum[x2][y1] + sum[x2][y2];
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  t.assign(2*k+2, vector<int>(2*k+2, 0));
  for(int i=0;i<n;i++) {
    ll x, y;
    char c;
    cin >> x >> y >> c;
    if(c == 'B') y += k;
    x %= (2 * k);
    y %= (2 * k);
    t[x][y]++;
  }
  sum.assign(2*k+2, vector<int>(2*k+2, 0));
  for(int i=0;i<2*k+1;i++) for(int j=0;j<2*k+1;j++) {
    sum[i+1][j+1] = t[i][j];
    sum[i+1][j+1] += sum[i+1][j];
    sum[i+1][j+1] += sum[i][j+1];
    sum[i+1][j+1] -= sum[i][j];
  }
  int ans = 0;
  for(int i=0;i<k;i++) for(int j=0;j<k;j++) {
    int res = get_sum(i, j, 0, 0);
    res += get_sum(i+k, j+k, i, j);
    res += get_sum(2*k, 2*k, i+k, j+k);
    res += get_sum(2*k, j, i+k, 0);
    res += get_sum(i, 2*k, 0, j+k);
    ans = max(ans, res);
    ans = max(ans, n-res);
  }
  cout << ans << endl;
  return 0;
}
