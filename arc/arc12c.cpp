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
bool five(vector<vector<int>> &t) {
  int n = t.size();
  vector<int> dx = {1, 0, 1, -1};
  vector<int> dy = {0, 1, 1, 1};
  auto check = [&](int x, int y) {
    return x >= 0 && y >= 0 && x < n && y < n;
  };
  for(int turn=0;turn<2;turn++) {
    for(int j=0;j<n;j++) for(int k=0;k<n;k++) {
      if(t[j][k] != turn) continue;
      for(int l=0;l<4;l++) {
        int y = j, x = k;
        if(check(x-dx[l], y-dy[l]) && t[y-dy[l]][x-dx[l]] == turn) continue;
        int m = 0;
        while(check(x, y)) {
          if(t[y][x] != turn) break;
          m++;
          y += dy[l];
          x += dx[l];
        }
        if(m >= 5) return false;
      }
    }
  }
  return true;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  const int n = 19;
  vector<string> b(n);
  int f = 0, s = 0;
  vector<vector<int>> t(n, vector<int>(n, -1));
  for(int i=0;i<n;i++) {
    cin >> b[i];
    for(int j=0;j<n;j++) {
      if(b[i][j] == 'o') f++, t[i][j] = 0;
      else if(b[i][j] == 'x') s++, t[i][j] = 1;
    }
  }
  if(!f && !s) {
    cout << "YES" << endl;
    return 0;
  }
  if(s > f || f > s+1) {
    cout << "NO" << endl;
    return 0;
  }
  int turn = !(f > s);
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    if(t[i][j] != turn) continue;
    t[i][j] = -1;
    if(five(t)) {
      cout << "YES" << endl;
      return 0;
    }
    t[i][j] = turn;
  }
  cout << "NO" << endl;
  return 0;
}
