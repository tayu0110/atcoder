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
  string s;
  cin >> n >> s;
  n = s.length();
  vector<int> v;
  int pos = 0;
  while(pos < n) {
    if(s[pos] != 'A') {
      pos++;
      continue;
    }
    int ac = 0;
    while(pos < n && s[pos] == 'A') ac++, pos++;
    if(pos == n) break;
    if(s[pos] != 'R') {
      if(s[pos] != 'A') pos++;
      continue;
    }
    pos++;
    if(pos == n) break;
    if(s[pos] != 'C') {
      if(s[pos] != 'A') pos++;
      continue;
    }
    int cc = 0;
    while(pos < n && s[pos] == 'C') cc++, pos++;
    if(min(ac, cc) == 0) continue;
    v.push_back(min(ac, cc));
  }
  sort(v.begin(), v.end());
  bool is_even = false;
  pos = 0;
  int ans = 0;
  while(pos < v.size()) {
    if(is_even) {
      is_even = false;
      pos++;
      ans++;
      continue;
    }
    v[v.size()-1]--;
    ans++;
    if(v[v.size()-1] == 0) {
      v.pop_back();
    } else if(v[v.size()-1] == 1) {
      ans++;
      v.pop_back();
      continue;
    }
    is_even = true;
  }
  cout << ans << endl;
  return 0;
}
