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
  string s;
  cin >> s;
  map<char, int> mp;
  mp['A'] = 0;
  mp['T'] = 1;
  mp['C'] = 2;
  mp['G'] = 3;
  vector<vector<int>> t(4, vector<int>(n+1, 0));
  for(int i=1;i<n+1;i++) {
    t[mp[s[i-1]]][i]++;
    for(int j=0;j<4;j++) {
      t[j][i] += t[j][i-1];
    }
  }
  int ans = 0;
  for(int i=0;i<n+1;i++) for(int j=i+1;j<n+1;j++) {
    int A = t[0][j] - t[0][i];
    int T = t[1][j] - t[1][i];
    int C = t[2][j] - t[2][i];
    int G = t[3][j] - t[3][i];
    if(A == T && C == G) ans++;
  }
  cout << ans << endl;
  return 0;
}
