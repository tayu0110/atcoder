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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]){
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<string> c(n);
  for(int i=0;i<n;i++) cin >> c[i];
  vector<vector<bool>> nc(n, vector<bool>(n, false));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) if(c[i][j] == '#') nc[i][j] = true;
  vector<vector<bool>> a(n, vector<bool>(n, false));
  vector<int> dx = {0, 1, 0, -1};
  vector<int> dy = {1, 0, -1, 0};
  for(int i=1;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(nc[i-1][j]) {
        a[i][j] = true;
        for(int k=0;k<4;k++) {
          int ni = i + dy[k];
          int nj = j + dx[k];
          if(ni < 0 || n <= ni || nj < 0 || n <= nj) continue;
          nc[ni][nj] = !nc[ni][nj];
        }
      }
    }
  }
  for(int i=0;i<n;i++) {
    for(int j=0;j<n;j++) {
      if(a[i][j]) cout << "#";
      else cout << ".";
    }
    cout << endl;
  }
  return 0;
}
