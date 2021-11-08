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
  string s, t;
  cin >> s >> t;
  int ans = 0;
  for(int i=0;i<s.length();i++) {
    if(s[i] == t[i]) continue;
    if(i == s.length()-1) {
      cout << "No" << endl;
      return 0;
    }
    swap(s[i], s[i+1]);
    ans++;
    if(s[i] != t[i]) {
      cout << "No" << endl;
      return 0;
    }
  }
  if(ans < 2) cout << "Yes" << endl;
  else cout << "No" << endl;
  return 0;
}
