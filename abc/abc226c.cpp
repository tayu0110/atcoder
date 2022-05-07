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
vector<bool> ck;
ll solve(int now, vector<ll> &t, vector<vector<int>> &a) {
  if(ck[now]) return 0;
  ck[now] = true;
  int n = t.size();
  ll res = 0;
  for(auto e : a[now]) {
    res += solve(e, t, a);
  }
  res += t[now];
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  vector<ll> t(n);
  vector<vector<int>> a(n);
  for(int i=0;i<n;i++) {
    cin >> t[i];
    int k;
    cin >> k;
    for(int j=0;j<k;j++) {
      int b;
      cin >> b;
      b--;
      a[i].push_back(b);
    }
  }
  ck.assign(n, false);
  cout << solve(n-1, t, a) << endl;
  return 0;
}
