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
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  for(int i=1;i<=t;i++) {
    int r, c;
    cin >> r >> c;
    vector<string> res;
    for(int i=0;i<r;i++) {
      string frame, hole;
      for(int j=0;j<c;j++) {
        frame += "+-";
        hole += "|.";
      }
      frame += "+";
      hole += "|";
      res.push_back(frame);
      res.push_back(hole);
      if(i == r-1) res.push_back(frame);
    }
    res[0][0] = '.';
    res[0][1] = '.';
    res[1][0] = '.';
    cout << "Case #" << i << ":" << endl;
    for(auto str : res) cout << str << endl;
  }
  return 0;
}
