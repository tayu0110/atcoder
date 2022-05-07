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
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    string s;
    cin >> s;
    string res;
    for(int j=0;j<s.length();j++) {
      for(int k=j+1;k<s.length();k++) {
        if(s[j] == s[k]) continue;
        if(s[j] < s[k]) {
          res += s[j];
        }
        break;
      }
      res += s[j];
    }
    printf("Case #%d: %s\n", i, res.c_str());
  }
  return 0;
}
