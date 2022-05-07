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
  int h, w;
  cin >> h >> w;
  vector<vector<int>> t(h, vector<int>(w, 0));
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    char c;
    cin >> c;
    if('1' <= c && c <= '5') t[i][j] = c - '0';
    else t[i][j] = -1;
  }
  vector<int> dx = {0, 1, 0, -1};
  vector<int> dy = {1, 0, -1, 0};
  for(int i=0;i<h;i++) for(int j=0;j<w;j++) {
    if(t[i][j] >= 0) continue;
    set<int> ck;
    for(int k=0;k<4;k++) {
      int nx = j + dx[k];
      int ny = i + dy[k];
      if(nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
      ck.insert(t[ny][nx]);
    }
    for(int k=1;k<6;k++) {
      if(ck.find(k) == ck.end()) {
        t[i][j] = k;
        break;
      }
    }
  }
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) cout << t[i][j];
    cout << endl;
  }
  return 0;
}
