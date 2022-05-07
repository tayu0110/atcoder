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
vector<int> dx = {0, 1, 1, -1};
vector<int> dy = {1, 0, 1, 1};
int n;
bool outofrange(int x, int y) {
  return x < 0 || x >= n || y < 0 || y >= n;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  cin >> n;
  vector<string> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) {
    for(int k=0;k<4;k++) {
      int nx = i-dx[k], ny = j-dy[k];
      int cnt = 0;
      for(int l=0;l<6;l++) {
        nx += dx[k];
        ny += dy[k];
        if(outofrange(nx, ny)) break;
        if(s[nx][ny] == '#') cnt++;
      }
      if(outofrange(nx, ny)) continue;
      DEBUG_EN("reached");
      if(cnt >= 4) {
        cout << "Yes" << endl;
        return 0;
      }
    }
  }
  cout << "No" << endl;
  return 0;
}
