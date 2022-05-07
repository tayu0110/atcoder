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

int main(int argc, char* argv[]){
  cin.tie(0);
  ios::sync_with_stdio(0);
  cout << fixed << setprecision(20);
  int t;
  cin >> t;
  vector<string> res;
  while(t--) {
    string a, s;
    cin >> a >> s;
    int na = a.length()-1, ns = s.length()-1;
    string ans;
    bool f = true;
    while(na >= 0 && ns >= 0) {
      int numa = a[na] - '0';
      int nums = s[ns] - '0';
      if(nums >= numa) {
        ans = (char)(nums-numa + '0') + ans;
      } else {
        if(ns == 0) {
          f = false;
          break;
        }
        ns--;
        nums += 10 * (s[ns] - '0');
        if(nums - numa > 9 || nums - numa < 0) {
          f = false;
          break;
        }
        ans = (char)(nums-numa + '0') + ans;
      }
      na--;
      ns--;
    }
    if(!f) res.push_back("-1");
    else if(na >= 0) res.push_back("-1");
    else {
      while(ns >= 0) ans = s[ns--] + ans;
      while(ans.length() > 1 && ans[0] == '0') ans = ans.substr(1);
      res.push_back(ans);
    }
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
