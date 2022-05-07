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
  string x;
  cin >> x;
  vector<ll> t(x.length());
  for(int i=0;i<x.length();i++) {
    if(i == 0) t[i] = x[i] - '0';
    else t[i] = t[i-1] + (x[i] - '0');
  }
  string ans;
  ll carry = 0;
  for(int i=x.length()-1;i>=0;i--) {
    ll tmp = t[i] + (carry % 10);
    ans += (char)((tmp % 10) + '0');
    carry /= 10;
    tmp /= 10;
    carry += tmp;
    // DEBUG(tmp);DEBUG_EN(carry);
  }
  while(carry) {
    ans += (char)((carry % 10) + '0');
    carry /= 10;
  }
  reverse(ans.begin(), ans.end());
  cout << ans << endl;
  return 0;
}
