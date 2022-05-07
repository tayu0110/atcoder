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
  int h, w, n;
  cin >> h >> w >> n;
  vector<int> a(n);
  for(int i=0;i<n;i++) cin >> a[i];
  vector<vector<int>> t(h, vector<int>(w, -1));
  int now = 0;
  for(int i=0;i<h;i++) {
    if(i % 2 == 0) {
      for(int j=0;j<w;j++) {
        while(!a[now]) now++;
        t[i][j] = now+1;
        a[now]--;
      }
    } else {
      for(int j=w-1;j>=0;j--) {
        while(!a[now]) now++;
        t[i][j] = now+1;
        a[now]--;
      }
    }
  }
  for(int i=0;i<h;i++) {
    for(int j=0;j<w;j++) {
      cout << t[i][j];
      if(j == w-1) cout << endl;
      else cout << " ";
    }
  }
  return 0;
}
