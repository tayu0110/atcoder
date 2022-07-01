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
bool check(string s, int div, int k) {
  int len = s.length();
  for(int i=0;i<div;i++) {
    int pos = i;
    int cnt = 0;
    vector<int> c(26);
    while(pos < len) {
      c[s[pos]-'a']++;
      pos += div;
      cnt++;
    }
    cnt -= *max_element(c.begin(), c.end());
    k -= cnt;
  }
  return k >= 0;
}
int main(int argc, char* argv[]) {
  cout << fixed << setprecision(20);
  int n, k;
  cin >> n >> k;
  string s;
  cin >> s;
  for(int i=1;i<=n;i++) {
    if(n % i == 0) {
      if(check(s, i, k)) {
        cout << i << endl;
        return 0;
      }
    }
  }
  return 0;
}
