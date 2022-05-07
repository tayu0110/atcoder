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
  int n, l, v, d;
  cin >> n >> l >> v >> d;
  vector<tuple<int, int, int>> p(n+1);
  p[0] = {0, v, d};
  for(int i=1;i<n+1;i++) {
    int x, v, d;
    cin >> x >> v >> d;
    p[i] = {x, v, d};
  }
  sort(p.begin(), p.end());
  n++;
  vector<vector<ld>> ans(n+1, vector<ld>(n, 0));
  for(int i=0;i<n;i++) {
    for(int j=0;j<i;j++) {
      
    }
  }
  return 0;
}
