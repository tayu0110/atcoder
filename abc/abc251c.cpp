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
template<class T> void print_with_space(T p) { for(auto e : p) cerr << e << " "; cerr << endl; }

const ll MOD = 1e9 + 7;
const ll INF = 1LL << 60;
const int inf = 1 << 29;

int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n;
  cin >> n;
  set<string> ck;
  int mx = -1;
  int ans = -1;
  for(int i=0;i<n;i++) {
    string s;
    int t;
    cin >> s >> t;
    if(ck.find(s) != ck.end()) continue;
    ck.insert(s);
    if(mx < t) {
      mx = t;
      ans = i+1;
    }
  }
  cout << ans << endl;
  return 0;
}
