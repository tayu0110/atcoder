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
  int n, a, b;
  cin >> n >> a >> b;
  char c = '.';
  vector<vector<char>> ans(n*a, vector<char>(n*b));
  for(int i=0;i<n*a;i+=a) {
    for(int j=0;j<n*b;j+=b) {
      if(i) c = ans[i-1][j] ^ '.' ^ '#';
      else if(j) c = ans[i][j-1] ^ '.' ^ '#';
      for(int k=0;k<a;k++) {
        for(int l=0;l<b;l++) {
          ans[i+k][j+l] = c;
        }
      }
    }
  }
  for(auto v : ans) {
    for(int i=0;i<v.size();i++) {
      cout << v[i];
    }
    cout << endl;
  }
  return 0;
}
