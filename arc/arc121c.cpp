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
  int t;
  cin >> t;
  while(t--) {
    int n;
    cin >> n;
    vector<int> p(n);
    for(int i=0;i<n;i++) cin >> p[i], p[i]--;
    vector<int> pos(n);
    for(int i=0;i<n;i++) pos[p[i]] = i;
    vector<int> ans;
    int k = n-1;
    while(k > 2) {
      if(pos[k] == k) {
        k--;
        continue;
      }
      while(pos[k] % 2 != ans.size() % 2) {
        int prev = 0;
        int nxt = 1;
        if(ans.size() % 2) prev++, nxt++;
        swap(pos[p[prev]], pos[p[nxt]]);
        swap(p[prev], p[nxt]);
        ans.push_back(prev+1);
      }
      int now = pos[k];
      int nxt = pos[k]+1;
      while(pos[k] != k) {
        ans.push_back(now+1);
        swap(pos[k], pos[p[nxt]]);
        swap(p[now], p[nxt]);
        now = nxt;
        nxt++;
      }
      k--;
    }
    if(k == 2) {
      int now = ans.size() % 2;
      while(p[0] != 0 || p[1] != 1 || p[2] != 2) {
        swap(p[now], p[now+1]);
        ans.push_back(now+1);
        now = (now+1) % 2;
      }
    } else {
      if(p[0] != 0) ans.push_back(1);
    }
    cout << ans.size() << endl;
    for(int i=0;i<ans.size();i++) {
      cout << ans[i];
      if(i != ans.size()-1) cout << " ";
    }
    cout << endl;
  }
  return 0;
}
