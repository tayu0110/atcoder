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
  int n, k;
  string s;
  cin >> n >> k >> s;
  if(k < 2) {
    cout << s << endl;
    return 0;
  }
  map<char, int> mp, smp;
  for(int i=0;i<n;i++) mp[s[i]]++, smp[s[i]]++;
  string ans;
  for(int i=0;i<n;i++) {
    smp[s[i]]--;
    k--;
    for(auto [fi, se] : mp) {
      if(fi == s[i]) {
        ans += fi;
        mp[fi]--;
        k++;
        break;
      }
      mp[fi]--;
      int match = 0;
      for(auto [f, s] : mp) match += min(s, smp[f]);
      if(k >= n-1-i-match) {
        ans += fi;
        break;
      }
      mp[fi]++;
    }
    if(!mp[ans[i]]) mp.erase(ans[i]);
  }
  cout << ans << endl;
  return 0;
}
