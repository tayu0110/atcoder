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
  string s;
  cin >> s;
  int l = 0, r = 0;
  int ans = 0;
  while(l < s.length()) {
    if(s[l] != 'A' && s[l] != 'G' && s[l] != 'C' && s[l] != 'T') {
      l++;
      continue;
    }
    r = l+1;
    while(r < s.length() && (s[r] == 'A' || s[r] == 'G' || s[r] == 'C' || s[r] == 'T')) r++;
    ans = max(ans, r-l);
    l = r;
  }
  cout << ans << endl;
  return 0;
}
