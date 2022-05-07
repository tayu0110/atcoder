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

using namespace std;

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
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  vector<string> s(n);
  for(int i=0;i<n;i++) cin >> s[i];
  int ans = 0;
  for(int i=1;i<(1<<n);i++) {
    vector<int> c(26);
    for(int j=0;j<n;j++) {
      if(i & (1 << j)) {
        for(auto e : s[j]) c[e-'a']++;
      }
    }
    int res = 0;
    for(auto e : c) {
      if(e == k) res++;
    }
    ans = max(ans, res);
  }
  cout << ans << endl;
  return 0;
}
