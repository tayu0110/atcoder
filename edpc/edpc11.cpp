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
vector<vector<int>> memo;
int rec(int k, int t, vector<int> &a) {
  if(memo[t][k] >= 0) return memo[t][k];
  int res = 0;
  for(auto e : a) {
    if(e > k) break;
    int r = rec(k-e, (t+1)%2, a);
    if(!t) res |= r;
    else res |= !r;
  }
  if(!t) return memo[t][k] = res;
  else return memo[t][k] = !res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  sort(a.begin(), a.end());
  memo.assign(2, vector<int>(k+1, -1));
  int ans = rec(k, 0, a);
  if(ans) cout << "First" << endl;
  else cout << "Second" << endl;
  return 0;
}