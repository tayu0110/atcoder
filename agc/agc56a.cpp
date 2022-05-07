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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<vector<char>> t(n, vector<char>(n, '.'));
  int m = n / 3 * 3;
  int v = 0, h = 0;
  int cnt = 0;
  while(h < m) {
    t[v][h] = '#';
    cnt++;
    if(cnt % 3 == 0) h++;
    v++;
    if(v == m) v = 0;
  }
  if(n % 3 == 1) {
    t[n-1][n-1] = '#';
    t[n-2][n-1] = '#';
    t[n-3][n-1] = '#';
    t[n-1][n-2] = '#';
    t[n-1][n-3] = '#';
    t[m-4][m-1] = '#';
    t[m-1][m-1] = '.';
    t[m-2][m-1] = '.';
    t[m-4][m-2] = '.';
  } else if(n % 3 == 2) {
    t[m-4][m-2] = '.';
    t[m-4][m-1] = '#';
    t[m-3][m-1] = '.';
    t[m-2][m-1] = '.';
    t[m-1][m-1] = '.';
    t[m-3][m] = '#';
    t[m-2][m] = '#';
    t[m][m-2] = '#';
    t[m][m-1] = '#';
    t[m+1][m-1] = '#';
    t[m+1][m] = '#';
    t[m+1][m+1] = '#';
    t[m][m+1] = '#';
    t[m-1][m+1] = '#';
  }
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) cout << t[i][j];
    cout << endl;
  }
  return 0;
}
