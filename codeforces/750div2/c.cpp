#include<iostream>
#include<iomanip>
#include<string>
#include<vector>
#include<algorithm>
#include<utility>
#include<tuple>
#include<map>
#include<queue>
#include<deque>
#include<set>
#include<stack>
#include<numeric>
#include<cstdio>
#include<cstdlib>
#include<cstring>
#include<cmath>
#include<cassert>

using namespace std;

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
int solve(char c, string &s) {
  int n = s.length();
  int l = 0, r = n-1;
  int res = 0;
  while(l < r) {
    if(s[l] == s[r]) {
      l++; r--;
      continue;
    }
    if(s[l] == c) {
      res++;
      l++;
    } else if(s[r] == c) {
      res++;
      r--;
    } else {
      return inf;
    }
  }
  return res;
}
int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<int> res;
  while(t--) {
    int n;
    cin >> n;
    string s;
    cin >> s;
    bool f = true;
    for(int i=0;i<n;i++) {
      int j = n-1 - i;
      if(i > j) break;
      if(s[i] != s[j]) f = false;
    }
    if(f) {
      res.push_back(0);
      continue;
    }
    int mn = inf;
    for(char c='a';c<='z';c++) {
      mn = min(mn, solve(c, s));
    }
    if(mn == inf) res.push_back(-1);
    else res.push_back(mn);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
