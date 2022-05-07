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
#include <bitset>

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
  int h, w, a, b;
  cin >> h >> w >> a >> b;
  int t = h * w;
  int dx[] = {0, 1};
  int dy[] = {1, 0};
  map<vector<vector<int>>, int> mp;
  mp[vector<vector<int>>(h, vector<int>(w, 0))];
  for(int i=0;i<a;i++) {
    int ni = i+1;
    map<vector<vector<int>>, int> tmp;
    for(auto [v, s] : mp) {
      tmp[v] = 1;
      for(int j=0;j<h;j++) for(int k=0;k<w;k++) {
        if(v[j][k]) continue;
        for(int l=0;l<2;l++) {
          int nj = j + dy[l];
          int nk = k + dx[l];
          if(v[nj][nk]) continue;
          
        }
      }
    }
  }
  return 0;
}
