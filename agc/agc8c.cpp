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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll i, o, t, j, l, s, z;
  cin >> i >> o >> t >> j >> l >> s >> z;
  if(i % 2 == 0 && j % 2 == 0 && l % 2 == 0) {
    cout << o + i + j + l << endl;
  } else {
    ll ans = o + (i / 2 * 2) + (j / 2 * 2) + (l / 2 * 2);
    if (i % 2 && j % 2 && l % 2) ans += 3;
    else if(i && j % 2 && l % 2) ans += 1;
    else if(i % 2 && j && l % 2) ans += 1;
    else if(i % 2 && j % 2 && l) ans += 1;
    cout << ans << endl;
  }
  return 0;
}
