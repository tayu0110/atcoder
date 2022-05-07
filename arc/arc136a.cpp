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
  int n;
  string s;
  cin >> n >> s;
  string t;
  for(int i=0;i<n;i++) {
    if(s[i] == 'A') t += "BB";
    else t += s[i];
  }
  DEBUG_EN(t);
  string res;
  for(int i=0;i<t.length();i++) {
    if(i == t.length()-1) res += t[i];
    else {
      if(t[i] == 'B' && t[i+1] == 'B') {
        res += "A";
        i++;
      } else {
        res += t[i];
      }
    }
  }
  cout << res << endl;
  return 0;
}
