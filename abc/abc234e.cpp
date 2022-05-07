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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  ll x;
  cin >> x;
  string s = to_string(x);
  if(s.length() < 3) {
    cout << x << endl;
    return 0;
  }
  int k = 10 * (s[0] - '0') + (s[1] - '0');
  for(int i=k;i<=111;i++) {
    string t = to_string(i);
    int diff = t[1] - t[0];
    int over = (i >= 100);
    while(t.length() < s.length() + over) {
      int now = t.back() - '0';
      now += diff;
      if(now > 9 || now < 0) break;
      t += (char)(now + '0');
    }
    if(t.length() < s.length()) continue;
    ll m = stoll(t);
    if(m < x) continue;
    cout << m << endl;
    return 0;
  }
  return 0;
}
