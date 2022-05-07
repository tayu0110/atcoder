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
#include <bitset>

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
  const int n = 4;
  vector<vector<int>> a(n, vector<int>(n, 0));
  int t = 0;
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    cin >> a[i][j];
    t = (t << 1) | a[i][j];
  }
  int ans = 0;
  int dx[] = {0, 1, 0, -1};
  int dy[] = {1, 0, -1, 0};
  vector<vector<int>> b(n, vector<int>(n, 0));
  for(int i=0;i<(1<<16);i++) {
    if((i & t) != t) continue;
    pii start;
    for(int j=0;j<n*n;j++) {
      b[j/n][j%n] = !!(i & (1<<j));
      if(b[j/n][j%n]) start = {j/n, j%n};
    }
    bool flag = true;
    for(int j=0;j<n-1;j++) for(int k=0;k<n-1;k++) {
      if(b[j][k] == b[j+1][k+1] && b[j+1][k] == b[j][k+1] && b[j][k] != b[j+1][k]) flag = false;
    }
    if(!flag) continue;
    int k = 0;
    queue<pii> nt;
    nt.push(start);
    while(!nt.empty()) {
      auto [y, x] = nt.front(); nt.pop();
      int id = y*n + x;
      if(k & (1 << id)) continue;
      k |= 1 << id;
      for(int j=0;j<4;j++) {
        int nx = x + dx[j];
        int ny = y + dy[j];
        if(nx < 0 || nx >= n || ny < 0 || ny >= n) continue;
        if(!b[ny][nx]) continue;
        nt.push({ny, nx});
      }
    }
    if(k != i) continue;
    flag = true;
    for(int j=0;j<n*n;j++) {
      int y = j / n, x = j % n;
      if(b[y][x]) continue;
      bool f = false;
      nt.push({j/n, j%n});
      while(!nt.empty()) {
        auto [y, x] = nt.front();
        nt.pop();
        if(b[y][x] == 1) continue;
        b[y][x] = 1;
        for(int k=0;k<4;k++) {
          int nx = x + dx[k];
          int ny = y + dy[k];
          if(nx < 0 || nx >= n || ny < 0 || ny >= n) {
            f = true;
            continue;
          }
          if(b[ny][nx]) continue;
          nt.push({ny, nx});
        }
      }
      if(!f) flag = false;
    }
    if(flag) ans++;
  }
  cout << ans << endl;
  return 0;
}
