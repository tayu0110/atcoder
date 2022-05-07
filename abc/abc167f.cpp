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
  int n;
  cin >> n;
  int res = 0;
  vector<tuple<int, int, int>> p, q;
  for(int i=0;i<n;i++) {
    int f = 0, b = 0, mn = inf, now = 0;
    int x = -inf, y = -inf;
    string s;
    cin >> s;
    for(int j=0;j<s.length();j++) {
      if(s[j] == '(') f++, now++;
      else b++, now--;
      x = max(x, b-f);
      mn = min(mn, now);
    }
    if(!now && mn >= 0) continue;
    f = 0, b = 0;
    for(int j=s.length()-1;j>=0;j--) {
      if(s[j] == '(') f++;
      else b++;
      y = max(y, f-b);
    }
    if(y-x >= 0) p.push_back({x, mn, now});
    else q.push_back({y, mn, now});
  }
  sort(p.begin(), p.end());
  sort(q.begin(), q.end(), greater<tuple<int, int, int>>());
  for(auto [_, mn, now] : p) {
    if(res+mn < 0) {
      cout << "No" << endl;
      return 0;
    }
    res += now;
  }
  for(auto [_, mn, now] : q) {
    if(res+mn < 0) {
      cout << "No" << endl;
      return 0;
    }
    res += now;
  }
  if(!res) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
