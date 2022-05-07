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
vector<int> dx = {1, 0, -1, 0};
vector<int> dy = {0, 1, 0, -1};
void update(int r, int c, queue<pii> &nt, vector<vector<bool>> &ck, vector<vector<int>> &t) {
  int n = t.size();
  int add = (ck[r][c] ? 0 : 1);
  for(int i=0;i<4;i++) {
    int ny = r + dy[i];
    int nx = c + dx[i];
    if(nx < 0 || nx >= n || ny < 0 || ny >= n) continue;
    if(t[ny][nx] <= t[r][c] + add) continue;
    t[ny][nx] = t[r][c] + add;
    nt.push({ny, nx});
  }
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<int> p(n*n);
  for(int i=0;i<n*n;i++) cin >> p[i], p[i]--;
  vector<vector<int>> t(n, vector<int>(n));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    t[i][j] = min(i, min(j, min(n-1-i, n-1-j)));
  }
  vector<vector<bool>> ck(n, vector<bool>(n, false));
  int ans = 0;
  for(int i=0;i<n*n;i++) {
    int r = p[i] / n;
    int c = p[i] % n;
    ans += t[r][c];
    ck[r][c] = true;
    queue<pii> nt;
    update(r, c, nt, ck, t);
    while(!nt.empty()) {
      auto [y, x] = nt.front();
      nt.pop();
      update(y, x, nt, ck, t);
    }
  }
  cout << ans << endl;
  return 0;
}
