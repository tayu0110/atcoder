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
  string s;
  cin >> s;
  deque<char> nt;
  bool f = true;
  for(int i=0;i<s.length();i++) {
    if(s[i] =='R') {
      f = !f;
    } else {
      if(f) {
        if(nt.empty()) nt.push_back(s[i]);
        else {
          if(nt.back() == s[i]) nt.pop_back();
          else nt.push_back(s[i]);
        }
      } else {
        if(nt.empty()) nt.push_front(s[i]);
        else {
          if(nt.front() == s[i]) nt.pop_front();
          else nt.push_front(s[i]);
        }
      }
    }
  }
  if(f) {
    while(!nt.empty()) {
      cout << nt.front();
      nt.pop_front();
    }
  } else {
    while(!nt.empty()) {
      cout << nt.back();
      nt.pop_back();
    }
  }
  cout << endl;
  return 0;
}
