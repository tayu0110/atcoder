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
  ll k;
  cin >> n >> k;
  vector<vector<ll>> t(n, vector<ll>(n, 0));
  for(int i=0;i<n;i++) for(int j=0;j<n;j++) cin >> t[i][j];
  vector<int> p(n);
  for(int i=0;i<n;i++) p[i] = i;
  int ans = 0;
  do {
    if(p[0] != 0) continue;
    p.push_back(0);
    ll s = 0;
    for(int i=0;i<n;i++) {
      s += t[p[i]][p[i+1]];
    }
    if(s == k) ans++;
    p.pop_back();
  } while(next_permutation(p.begin(), p.end()));
  cout << ans << endl;
  return 0;
}
