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
  int n, q;
  cin >> n >> q;
  vector<int> prev(n, -1);
  vector<int> nt(n, -1);
  while(q--) {
    int t, x;
    cin >> t >> x;
    x--;
    if(t == 1) {
      int y;
      cin >> y;
      y--;
      prev[y] = x;
      nt[x] = y;
    } else if(t == 2) {
      int y;
      cin >> y;
      y--;
      prev[y] = -1;
      nt[x] = -1;
    } else {
      int top = x;
      while(prev[top] >= 0) {
        top = prev[top];
      }
      vector<int> res;
      do {
        res.push_back(top);
        top = nt[top];
      } while(top >= 0);
      cout << res.size();
      if(res.size() == 0) cout << endl;
      else cout << " ";
      for(int i=0;i<res.size();i++) {
        cout << res[i] + 1;
        if(i == res.size()-1) cout << endl;
        else cout << " ";
      }
    }
  }
  return 0;
}
