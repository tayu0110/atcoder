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
  int a, b, c, d;
  cin >> a >> b >> c >> d;
  for(int i=a;i<=b;i++) {
    bool win = true;
    for(int j=c;j<=d;j++) {
      int now = i+j;
      bool isprime = true;
      for(int k=2;k*k<=now;k++) {
        if(now % k) continue;
        isprime = false;
        break;
      }
      if(isprime) {
        win = false;
        break;
      }
    }
    if(win) {
      cout << "Takahashi" << endl;
      return 0;
    }
  }
  cout << "Aoki" << endl;
  return 0;
}
