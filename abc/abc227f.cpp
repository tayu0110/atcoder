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
vector<vector<vector<ll>>> memo;
vector<ll> rec(int r, int c, vector<vector<ll>> &a) {
  int h = a.size();
  int w = a[0].size();
  if(r == h || c == w) return vector<ll>();
  if(memo[r][c].size()) return memo[r][c];
  vector<ll> right = rec(r, c+1, a);
  vector<ll> down = rec(r+1, c, a);
  sort(right.begin(), right.end());
  sort(down.begin(), down.end());
  ll rr = 0;
  ll dr = 0;
  if(!right.size()) {
    rr = INF;
    
  } else if(!down.size()) {

  } else {

  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int h, w, k;
  cin >> h >> w >> k;
  vector<vector<ll>> a(h, vector<ll>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) cin >> a[i][j];
  memo.assign(h, vector<vector<ll>>(w, vector<ll>()));

  return 0;
}
