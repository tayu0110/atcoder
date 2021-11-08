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
  vector<int> res;
  while(t--) {
    ll n;
    cin >> n;
    if(n % 25 == 0) {
      res.push_back(0);
      continue;
    }
    vector<int> k(10);
    string s = to_string(n);
    int len = s.length();
    for(int i=0;i<s.length();i++) k[s[i]-'0']++;
    int ans = 0;
    for(int i=0;i<len;i++) if(s[i] != '0') ans++;
    if(k[1] && k[0] >= 2) {
      int o = 1, z = 2;
      int r = 0;
      for(int i=len-1;i>=0;i--) {
        if(s[i] == '0' && z) z--;
        else if(s[i] == '1' && o && !z) o--;
        else if(!z && !o) break;
        else r++;
      }
      ans = min(ans, r);
    }
    if(k[7] && k[5]) {
      int sev = 1, f = 1;
      int r = 0;
      for(int i=len-1;i>=0;i--) {
        if(s[i] == '5' && f) f--;
        else if(s[i] == '7' && sev && !f) sev--;
        else if(!sev && !f) break;
        else r++;
      }
      ans = min(ans, r);
    }
    if(k[5] && k[0]) {
      int f = 1, z = 1;
      int r = 0;
      for(int i=len-1;i>=0;i--) {
        if(s[i] == '0' && z) z--;
        else if(s[i] == '5' && f && !z) f--;
        else if(!z && !f) break;
        else r++;
      }
      ans = min(ans, r);
    }
    if(k[2] && k[5]) {
      int two = 1, f = 1;
      int r = 0;
      for(int i=len-1;i>=0;i--) {
        if(s[i] == '5' && f) f--;
        else if(s[i] == '2' && !f && two) two--;
        else if(!f && !two) break;
        else r++;
      }
      ans = min(ans, r);
    }
    if(k[0] >= 0) {
      int z = 2;
      int r = 0;
      for(int i=len-1;i>=0;i--) {
        if(s[i] == '0' && z) z--;
        else if(!z) break;
        else r++;
      }
      ans = min(ans, r);
    }
    res.push_back(ans);
  }
  for(auto e : res) cout << e << endl;
  return 0;
}
