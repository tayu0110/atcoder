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
  int n, m;
  cin >> n >> m;
  vector<vector<ll>> b(n, vector<ll>(m));
  for(int i=0;i<n;i++) for(int j=0;j<m;j++) cin >> b[i][j];
  bool f = true;
  for(int i=0;i<m;i++) for(int j=0;j<n-1;j++) {
    if(b[j+1][i] - b[j][i] != 7) f = false;
  }
  for(int i=0;i<n;i++) for(int j=0;j<m-1;j++) {
    ll nt = b[i][j+1] % 7;
    ll now = b[i][j] % 7;
    if(nt == 0) nt = 7;
    if(now == 0) now = 7;
    if(nt - now != 1) f = false;
  }
  if(f) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
