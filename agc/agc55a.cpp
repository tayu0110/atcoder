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
  int n;
  cin >> n;
  string s;
  cin >> s;
  vector<int> ans(3*n);
  string t = "ABC";
  int g = 1;
  do {
    vector<int> p(3, 0);
    for(int i=0;i<3;i++) for(int j=i*n;j<(i+1)*n;j++) if(t[i] == s[j] && !ans[j]) p[i]++;
    int mn = min({p[0], p[1], p[2]});
    p[0] = p[1] = p[2] = mn;
    for(int i=0;i<3;i++) for(int j=i*n;j<(i+1)*n;j++) if(t[i] == s[j] && !ans[j] && p[i]) {
      p[i]--;
      ans[j] = g;
    }
    g++;
  } while(next_permutation(t.begin(), t.end()));
  for(auto e : ans) cout << e;
  cout << endl;
  return 0;
}
